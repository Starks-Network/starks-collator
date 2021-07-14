
use super::*;
use crate::mock::*;
use sp_core::{traits::TaskExecutorExt, testing::TaskExecutor};
use sp_core::offchain::{
	OffchainDbExt,
	OffchainWorkerExt,
	TransactionPoolExt,
	testing::{self as testing, TestOffchainExt, TestTransactionPoolExt},
};
use frame_support::{assert_ok, traits::OnFinalize};
use sp_runtime::{testing::UintAuthorityId};
use frame_support::traits::OffchainWorker;
use sp_runtime::transaction_validity::TransactionSource;

#[test]
fn set_key_works() {
	new_test_ext().execute_with(|| {
		advance_session();
		VALIDATORS.with(|l| *l.borrow_mut() = Some(vec![1, 2, 3, 4, 5, 6]));
		assert_eq!(Session::validators(), Vec::<u64>::new());
		// enact the change and buffer another one
		advance_session();

		assert_eq!(Session::current_index(), 2);
		assert_eq!(Session::validators(), vec![1, 2, 3]);
	});
}

#[test]
fn should_rotate_keys() {
	let mut ext = new_test_ext();
	ext.execute_with( || {
		System::set_block_number(1);
		advance_session();
		assert_eq!(Session::validators(), Vec::<u64>::new());

		advance_session();
		assert_eq!(Session::validators(), vec![1, 2, 3]);
		assert_eq!(Verifier::keys(), vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)]);

		VALIDATORS.with(|l| *l.borrow_mut() = Some(vec![1, 2, 3, 4, 5, 6]));
		assert_eq!(Session::validators(), vec![1, 2, 3]);
 	});
}

#[test]
fn should_create_task() {
	let mut ext = new_test_ext();

	ext.execute_with( || {
		let (progam_hash, inputs, outputs, proof_id) = task_params();
		let class = b"class".to_vec();
		assert_ok!(Verifier::create_task(Origin::signed(1), class, progam_hash, inputs.clone(), outputs.clone(), proof_id.clone()));
		
		let class = b"class".to_vec();
		
		assert_eq!(
			Verifier::task_params(&1,&class),
			TaskInfo {
				proof_id: b"QmRLkDFVqqDNKUarn7x5Bfu2YdW9R1qznA78cP4XFuxfaa".to_vec(),
				inputs,
				outputs,
				program_hash:progam_hash
			}
		);

		assert_eq!(
			Verifier::ongoing_tasks(&1,&class),
			Some(Status {
				verifiers: Vec::<u32>::new(),
				ayes: 0,
				nays: 0
			})
		);
	})
}

#[test]
fn should_parse_http_response() {
	let (offchain, offchain_state) = TestOffchainExt::new();
	let mut ext = sp_io::TestExternalities::default();
	
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));
	ext.register_extension(TaskExecutorExt::new(TaskExecutor::new()));

	let (_, _, _, proof_id) = task_params();

	{
		let mut state = offchain_state.write();
		let uri = "https://ipfs.infura.io:5001/api/v0/cat?arg=".to_owned() + sp_std::str::from_utf8(&proof_id[..]).unwrap();
		state.expect_request(testing::PendingRequest {
			method: "GET".into(),
			uri: uri.clone(),
			response: new_proof().ok(),
			sent: true,
			..Default::default()
		});
	}

	ext.execute_with(|| {
		set_key_and_tasks();
		let proof = Verifier::fetch_proof(&proof_id);
		assert_eq!(proof.unwrap(), new_proof().unwrap());
	});
}


#[test]
fn basic_starks_verifier_works() {
	new_test_ext().execute_with(|| {
		// get proof
		let proof = new_proof().unwrap();
		let (prog_hash,inputs, outputs, _) = task_params();
		let res = sp_starks::starks::verify(&prog_hash, &inputs, &outputs, &proof);
		assert!(res.is_ok());
	});
}


#[test]
fn should_send_extrinsic() {
	let (offchain, offchain_state) = TestOffchainExt::new();
	let (pool, pool_state) = TestTransactionPoolExt::new();
	let mut ext = sp_io::TestExternalities::default();
	ext.register_extension(TransactionPoolExt::new(pool));
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));

	let (program_hash, _, _, _) = task_params();
	let class = b"class".to_vec();
	three_http_request(&mut offchain_state.write());

	ext.execute_with(|| {
		UintAuthorityId::set_all_keys(vec![1, 2, 3]);
		assert_eq!(Verifier::ongoing_tasks(&1, &class.clone()), None);
		set_key_and_tasks();
		assert_eq!(Session::validators(), vec![1, 2, 3]);
		assert_eq!(Verifier::keys(), vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)]);
	
		// Offchain worker
		Verifier::offchain_worker(System::block_number());

		// then
		let transaction = pool_state.write().transactions.pop().unwrap();
		// All validators have `0` as their session key, so we generate 2 transactions.
		assert_eq!(pool_state.read().transactions.len(), 2);

		// check the transaction
		let ex: Extrinsic = Decode::decode(&mut &*transaction).unwrap();
		let receipt = match ex.call {
			crate::mock::Call::Verifier(crate::Call::submit_verification(r, ..)) => r,
			e => panic!("Unexpected call: {:?}", e),
		};

		// pop the last one
		assert_eq!(receipt.clone(), VerificationReceipt {
			task_tuple_id: (1,b"class".to_vec()),
			program_hash: program_hash,
    		passed: true,
    		submit_at: System::block_number(),
    		auth_index: 2,
    		validators_len: 3
		});
		// test ValidateUnsigned
		let signature = UintAuthorityId(3).sign(&receipt.clone().encode()).unwrap();
		let call = crate::Call::submit_verification(receipt.clone(), signature.clone());
		assert_ok!(
			<Verifier as sp_runtime::traits::ValidateUnsigned>::validate_unsigned(
				TransactionSource::External,
				&call
			)
		);

		let _ = Verifier::submit_verification(Origin::none(), receipt, signature);

		// check the online status
		let status = Verifier::ongoing_tasks(&1,&class);
		assert_eq!(status, Some(Status {
			verifiers: vec![2],
			ayes: 1,
			nays: 0
		}));
			// then
			let transaction_1 = pool_state.write().transactions.pop().unwrap();
			assert_eq!(pool_state.read().transactions.len(), 1);

		// check the transaction
		let ex: Extrinsic = Decode::decode(&mut &*transaction_1).unwrap();
		let receipt = match ex.call {
			crate::mock::Call::Verifier(crate::Call::submit_verification(r, ..)) => r,
			e => panic!("Unexpected call: {:?}", e),
		};

		// pop the last one
		assert_eq!(receipt.clone(), VerificationReceipt {
			task_tuple_id: (1,class),
			program_hash: program_hash,
    		passed: true,
    		submit_at: System::block_number(),
    		auth_index: 1,
    		validators_len: 3
		});
		// test ValidateUnsigned
		let signature = UintAuthorityId(2).sign(&receipt.clone().encode()).unwrap();
		let call = crate::Call::submit_verification(receipt.clone(), signature.clone());
		assert_ok!(
			<Verifier as sp_runtime::traits::ValidateUnsigned>::validate_unsigned(
				TransactionSource::External,
				&call
			)
		);

		let _ = Verifier::submit_verification(Origin::none(), receipt, signature);

		let block_number = System::block_number() + 20;
		let class = b"class".to_vec();
		// check the online status, should be removed
		let status = Verifier::ongoing_tasks(&1,&class);
		assert_eq!(status, None);
		let settled_task = Verifier::settled_tasks(&block_number, &(1,b"class".to_vec()));
		assert_eq!(settled_task, true);

		Verifier::on_finalize(block_number);
		
		assert_eq!(Verifier::settled_tasks(&block_number, &(1,b"class".to_vec())), false);


	});
}

fn set_key_and_tasks() {

	// set keys
	System::set_block_number(1);
	advance_session();
	assert_eq!(Session::validators(), Vec::<u64>::new());

	advance_session();
	assert_eq!(Session::validators(), vec![1, 2, 3]);
	assert_eq!(Verifier::keys(), vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)]);
	// craete task
	let (progam_hash, inputs, outputs, proof_id) = task_params();
	let class = b"class".to_vec();
	assert_ok!(Verifier::create_task(Origin::signed(1), class, progam_hash.clone(), inputs.clone(), outputs.clone(), proof_id.clone()));
}

// return program_hash, inputs, outputs, proof_id
fn task_params() -> ([u8; 32], Vec<u128>, Vec<u128>, Vec<u8>) {
	(
		[19, 23, 145, 150, 7, 226, 183, 94, 42, 36, 220, 169, 148, 89, 125, 153, 113, 250, 202, 142, 187, 167, 14, 144, 186, 217, 89, 214, 222, 234, 43, 214],
		vec![1u128,0u128],
		vec![8u128],
		b"QmRLkDFVqqDNKUarn7x5Bfu2YdW9R1qznA78cP4XFuxfaa".to_vec()
	)
}

fn three_http_request(state: &mut testing::OffchainState)  {
	let (_, _, _, proof_id) = task_params();
	let uri = "https://ipfs.infura.io:5001/api/v0/cat?arg=".to_owned() + sp_std::str::from_utf8(&proof_id[..]).unwrap();
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: uri.clone(),
		response: new_proof().ok(),
		sent: true,
		..Default::default()
	});

	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: uri.clone(),
		response: new_proof().ok(),
		sent: true,
		..Default::default()
	});

	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: uri.clone(),
		response: new_proof().ok(),
		sent: true,
		..Default::default()
	});
}




 


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use ark_r1cs_std::R1CSVar;
    use simpleworks::types::value::SimpleworksValueType::{Address, Record, U128, U16, U32, U64};
    use snarkvm::prelude::{Identifier, Parser, Program, Testnet3};
    use vmtropy::circuit_io_type::CircuitIOType::SimpleRecord;

    fn read_add_program(instruction: &str) -> Result<String> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(&format!("programs/{instruction}/main.aleo"));
        let program = std::fs::read_to_string(path).unwrap_or_else(|_| "".to_owned());
        Ok(program)
    }

    #[test]
    fn test01_add_with_u16_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_1").unwrap())
            .unwrap();
        /*
        function hello_1:
            input r0 as u16.public;
            input r1 as u16.public;
            add r0 r1 into r2;
            output r2 as u16.public;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test02_add_with_u16_private_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_2").unwrap())
            .unwrap();

        /*
        function hello_2:
            input r0 as u16.private;
            input r1 as u16.private;
            add r0 r1 into r2;
            output r2 as u16.private;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test03_add_with_u16_private_and_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_3").unwrap())
            .unwrap();

        /*
        function hello_3:
            input r0 as u16.public;
            input r1 as u16.public;
            add r0 r1 into r2;
            output r2 as u16.private;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test04_add_with_u32_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_4").unwrap())
            .unwrap();

        /*
        function hello_4:
            input r0 as u32.public;
            input r1 as u32.public;
            add r0 r1 into r2;
            output r2 as u32.public;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test05_add_with_u32_private_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_5").unwrap())
            .unwrap();

        /*
        function hello_5:
            input r0 as u32.private;
            input r1 as u32.private;
            add r0 r1 into r2;
            output r2 as u32.private;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test06_add_with_u32_private_and_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_6").unwrap())
            .unwrap();

        /*
        function hello_6:
            input r0 as u32.public;
            input r1 as u32.public;
            add r0 r1 into r2;
            output r2 as u32.private;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test07_add_with_u64_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_7").unwrap())
            .unwrap();

        /*
        function hello_7:
            input r0 as u64.public;
            input r1 as u64.public;
            add r0 r1 into r2;
            output r2 as u64.public;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test08_add_with_u64_private_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_8").unwrap())
            .unwrap();

        /*
        function hello_8:
            input r0 as u64.private;
            input r1 as u64.private;
            add r0 r1 into r2;
            output r2 as u64.private;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test09_add_with_u64_private_and_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_9").unwrap())
            .unwrap();

        /*
        function hello_9:
            input r0 as u64.public;
            input r1 as u64.public;
            add r0 r1 into r2;
            output r2 as u64.private;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test10_add_with_u128_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_10").unwrap())
            .unwrap();

        /*
        function hello_10:
            input r0 as u128.public;
            input r1 as u128.public;
            add r0 r1 into r2;
            output r2 as u128.public;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test11_add_with_u128_private_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_11").unwrap())
            .unwrap();

        /*
        function hello_11:
            input r0 as u128.private;
            input r1 as u128.private;
            add r0 r1 into r2;
            output r2 as u128.private;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test12_add_with_u128_private_and_public_inputs() {
        let program_string = read_add_program("add").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_12").unwrap())
            .unwrap();

        /*
        function hello_12:
            input r0 as u128.public;
            input r1 as u128.public;
            add r0 r1 into r2;
            output r2 as u128.private;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u16_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_1").unwrap())
            .unwrap();

        /*
        function hello_1:
            input r0 as u16.public;
            input r1 as u16.public;
            add r0 r1 into r2;
            output r2 as u16.public;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u16_private_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_2").unwrap())
            .unwrap();

        /*
        function hello_2:
            input r0 as u16.private;
            input r1 as u16.private;
            add r0 r1 into r2;
            output r2 as u16.private;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u16_private_and_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_3").unwrap())
            .unwrap();

        /*
        function hello_3:
            input r0 as u16.public;
            input r1 as u16.public;
            add r0 r1 into r2;
            output r2 as u16.private;
        */

        let user_inputs = vec![U16(1), U16(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u32_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_4").unwrap())
            .unwrap();

        /*
        function hello_4:
            input r0 as u32.public;
            input r1 as u32.public;
            add r0 r1 into r2;
            output r2 as u32.public;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u32_private_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_5").unwrap())
            .unwrap();

        /*
        function hello_5:
            input r0 as u32.private;
            input r1 as u32.private;
            add r0 r1 into r2;
            output r2 as u32.private;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u32_private_and_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_6").unwrap())
            .unwrap();

        /*
        function hello_6:
            input r0 as u32.public;
            input r1 as u32.public;
            add r0 r1 into r2;
            output r2 as u32.private;
        */

        let user_inputs = vec![U32(1), U32(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u64_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_7").unwrap())
            .unwrap();

        /*
        function hello_7:
            input r0 as u64.public;
            input r1 as u64.public;
            add r0 r1 into r2;
            output r2 as u64.public;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u64_private_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_8").unwrap())
            .unwrap();

        /*
        function hello_8:
            input r0 as u64.private;
            input r1 as u64.private;
            add r0 r1 into r2;
            output r2 as u64.private;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_subtract_with_u64_private_and_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_9").unwrap())
            .unwrap();

        /*
        function hello_9:
            input r0 as u64.public;
            input r1 as u64.public;
            add r0 r1 into r2;
            output r2 as u64.private;
        */

        let user_inputs = vec![U64(1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test_subtract_with_u128_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_10").unwrap())
            .unwrap();

        /*
        function hello_10:
            input r0 as u128.public;
            input r1 as u128.public;
            add r0 r1 into r2;
            output r2 as u128.public;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test_subtract_with_u128_private_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_11").unwrap())
            .unwrap();

        /*
        function hello_11:
            input r0 as u128.private;
            input r1 as u128.private;
            add r0 r1 into r2;
            output r2 as u128.private;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    #[ignore = "U128 is supported in certain fields, TODO: Figure out if we want to support U128 operations"]
    fn test_subtract_with_u128_private_and_public_inputs() {
        let program_string = read_add_program("subtract").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_12").unwrap())
            .unwrap();

        /*
        function hello_12:
            input r0 as u128.public;
            input r1 as u128.public;
            add r0 r1 into r2;
            output r2 as u128.private;
        */

        let user_inputs = vec![U128(1), U128(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_record_add() {
        let program_string = read_add_program("record").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_1").unwrap())
            .unwrap();

        let mut address = [0_u8; 63];
        let address_string = "aleo1sk339wl3ch4ee5k3y6f6yrmvs9w63yfsmrs9w0wwkx5a9pgjqggqlkx5zh";
        for (address_byte, address_string_byte) in address.iter_mut().zip(address_string.as_bytes())
        {
            *address_byte = *address_string_byte;
        }

        let user_inputs = vec![Record(address, 0), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_record_subtract() {
        let program_string = read_add_program("record").unwrap();
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("hello_2").unwrap())
            .unwrap();

        let mut address = [0_u8; 63];
        let address_string = "aleo1sk339wl3ch4ee5k3y6f6yrmvs9w63yfsmrs9w0wwkx5a9pgjqggqlkx5zh";
        for (address_byte, address_string_byte) in address.iter_mut().zip(address_string.as_bytes())
        {
            *address_byte = *address_string_byte;
        }

        let user_inputs = vec![Record(address, 1), U64(1)];

        // execute circuit
        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();
        assert!(ret_ok);

        for (register, output) in circuit_outputs {
            println!("{}: {:?}", register, output.value().unwrap());
        }
    }

    #[test]
    fn test_credits_mint() {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("programs/credits.aleo");
        let program_string = std::fs::read_to_string(path).unwrap_or_else(|_| "".to_owned());
        let (_, program) = Program::<Testnet3>::parse(&program_string).unwrap();
        let function = program
            .get_function(&Identifier::try_from("mint").unwrap())
            .unwrap();

        let mut address = [0_u8; 63];
        let address_string = "aleo1sk339wl3ch4ee5k3y6f6yrmvs9w63yfsmrs9w0wwkx5a9pgjqggqlkx5zh";
        for (address_byte, address_string_byte) in address.iter_mut().zip(address_string.as_bytes())
        {
            *address_byte = *address_string_byte;
        }

        let user_inputs = vec![Address(address), U64(1)];

        let (ret_ok, circuit_outputs, _bytes_proof) =
            vmtropy::execute_function(function, &user_inputs).unwrap();

        let expected_output_register_locator = &"r2".to_string();
        assert!(ret_ok);
        assert!(circuit_outputs.len() == 1);
        if let (output_register_locator, SimpleRecord(record)) = circuit_outputs.first().unwrap() {
            assert_eq!(output_register_locator, expected_output_register_locator);
            assert_eq!(record.owner.value().unwrap(), address_string);
            assert_eq!(record.gates.value().unwrap(), 1);
        }
    }
}

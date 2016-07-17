use registers::Register;
use util::*;

#[test]
fn vm_mov() {
    let code = r#"
        mov $a0, $a1
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A0, 40);
            vm.write(Register::A1, 13);

            vm.run();

            assert_eq!(vm.read(Register::A0), 13);
            assert_eq!(vm.read(Register::A1), 13);
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_add() {
    let code = r#"
        add $a0, $a1, $a2
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A0, 0);
            vm.write(Register::A1, 7);
            vm.write(Register::A2, -2);

            vm.run();

            assert_eq!(vm.read(Register::A0), 5);
            assert_eq!(vm.read(Register::A1), 7);
            assert_eq!(vm.read(Register::A2), -2);
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_mul() {
    let code = r#"
        mul $a1, $a2
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::HI, 999);
            vm.write(Register::A1, -15);
            vm.write(Register::A2, -3);

            vm.run();

            assert_eq!(vm.read(Register::LO), 45);
            assert_eq!(vm.read(Register::HI), 0);
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_not() {
    let code = r#"
        not $a0, $a1
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A1, 84728860944); // 10T010T010T010T010T010T0

            vm.run();

            assert_eq!(vm.read(Register::A0), -84728860944); // T010T010T010T010T010T010
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_and() {
    let code = r#"
        and $a0, $a1, $a2
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A1, 141214768227); // 111111111111111111111000
            vm.write(Register::A2, 120294061834); // 11T111111111111111111111

            vm.run();

            assert_eq!(vm.read(Register::A0), 120294061821); // 11T111111111111111111000
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_or() {
    let code = r#"
        and $a0, $a1, $a2
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A1, -139492630440); // TTTT0000TTTT000011110000
            vm.write(Register::A2, 84728860944);   // 10T010T010T010T010T010T0

            vm.run();

            assert_eq!(vm.read(Register::A0), -104619473316); // 1TTT10T01TTT10T0111110T0
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_shf() {
    let code = r#"
        shf $a0, $a1, $a2
        halt
    "#;

    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.write(Register::A1, 141_214_768_240);            // 111111111111111111111111
            vm.write(Register::A2, 2);
            vm.write(Register::LO, 123);
            vm.write(Register::HI, 456);

            vm.run();

            assert_eq!(vm.read(Register::LO), 0);
            assert_eq!(vm.read(Register::HI), 4);               // 000000000000000000000011
            assert_eq!(vm.read(Register::A0), 141_214_768_236); // 111111111111111111111100
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

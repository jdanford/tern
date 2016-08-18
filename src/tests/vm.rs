use registers::Register;
use util::*;
use vm::VM;

fn test_program<F: Fn(&mut VM)>(code: &str, f: F) {
    match vm_from_code(code) {
        Ok(mut vm) => {
            vm.run();
            f(&mut vm);
        }

        Err(e) => {
            println!("{}", e);
            assert!(false);
        }
    }
}

#[test]
fn vm_mov() {
    let code = r#"
        __start:
            movi $a0, 40
            movi $a1, 13
            mov $a0, $a1
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), 13);
        assert_eq!(vm.read(Register::A1), 13);
    });
}

#[test]
fn vm_add() {
    let code = r#"
        __start:
            movi $a0, 0
            movi $a1, 7
            movi $a2, -2
            add $a0, $a1, $a2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), 5);
        assert_eq!(vm.read(Register::A1), 7);
        assert_eq!(vm.read(Register::A2), -2);
    });
}

#[test]
fn vm_mul() {
    let code = r#"
        __start:
            movi $hi, 999
            movi $a1, -15
            movi $a2, -3
            mul $a1, $a2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::LO), 45);
        assert_eq!(vm.read(Register::HI), 0);
    });
}

#[test]
fn vm_not() {
    let code = r#"
        __start:
            movw $a1, 0t10T010T010T010T010T010T0
            not $a0, $a1
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), -84728860944); // T010T010T010T010T010T010
    });
}

#[test]
fn vm_and() {
    let code = r#"
        __start:
            movw $a1, 0t111111111111111111111000
            movw $a2, 0t11T111111111111111111111
            and $a0, $a1, $a2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), 120294061821); // 11T111111111111111111000
    });
}

#[test]
fn vm_or() {
    let code = r#"
        __start:
            movw $a1, 0tTTTT0000TTTT000011110000
            movw $a2, 0t10T010T010T010T010T010T0
            and $a0, $a1, $a2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), -104619473316); // 1TTT10T01TTT10T0111110T0
    });
}

#[test]
fn vm_shf() {
    let code = r#"
        __start:
            movw $a1, 0t111111111111111111111111
            movi $a2, 2
            movi $lo, 123
            movi $hi, 456
            shf $a0, $a1, $a2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::LO), 0);
        assert_eq!(vm.read(Register::HI), 4);               // 000000000000000000000011
        assert_eq!(vm.read(Register::A0), 141_214_768_236); // 111111111111111111111100
    });
}

#[test]
fn vm_shfi() {
    let code = r#"
        __start:
            movw $a0, 0t111111111111111111111111
            movi $lo, 123
            movi $hi, 456
            shfi $a0, 2
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::LO), 0);
        assert_eq!(vm.read(Register::HI), 4);               // 000000000000000000000011
        assert_eq!(vm.read(Register::A0), 141_214_768_236); // 111111111111111111111100
    });
}

#[test]
fn vm_cmp() {
    let code = r#"
        __start:
            movi $a0, 0tT1
            movi $a1, 0t01
            movi $a2, 0t1T

            cmp $t0, $a0, $a1
            cmp $t1, $a1, $a2
            cmp $t2, $a2, $a0
            cmp $t3, $a2, $a2

            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::T0), -1);
        assert_eq!(vm.read(Register::T1), -1);
        assert_eq!(vm.read(Register::T2), 1);
        assert_eq!(vm.read(Register::T3), 0);
    });
}

#[test]
fn vm_jmp() {
    let code = r#"
        __start:
            movi $a0, 103
            jmp end

        garbage:
            movi $a0, 456

        end:
            addi $a0, 20
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), 123);
    });
}

#[test]
fn vm_jt() {
    let code = r#"
        __start:
            movi $a0, 103
            movi $a1, 0tT
            jT $a1, end

        garbage:
            movi $a0, 456

        end:
            addi $a0, 20
            halt
    "#;

    test_program(code, |ref mut vm| {
        assert_eq!(vm.read(Register::A0), 123);
    });
}

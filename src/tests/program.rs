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
fn program_jmp() {
    let code = r#"
    start:
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
fn program_jt() {
    let code = r#"
    start:
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

#[test]
fn program_cmp() {
    let code = r#"
    start:
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

use wasmedge_process_interface::Command;

fn main() {
    let cmd = Command::new("echo hello from rust").timeout(1000).output();

    println!(" return code : {}", cmd.status);
    println!(" stdout :");
    print!("{}", str::from_utf8(&cmd.stdout).expect("GET STDOUT ERR"));
    println!(" stderr :");
    print!("{}", str::from_utf8(&cmd.stderr).expect("GET STDERR ERR"));
}

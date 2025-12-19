fn main() {
    println!("Hello, panic!");
    /*
     * panic! can be called manually,
     * - prints failure message
     * - unwind, clean up the stack
     * - quiting of program
     * additionally if you set env var
     * displays the call stack when it happens
     * [profile.(type eg. release)]
     * panic = 'abort' -> quit without cleaning
    */
    panic!("CRASHING AND BURNING");
}

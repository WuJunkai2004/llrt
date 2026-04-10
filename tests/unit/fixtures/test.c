/**
 * compile the file to wasm
 * and test if WebAssembly is supported wholely by the runtime, without any helper functions from the host.
 * compile by command:
 * > clang --target=wasm32-unknown-unknown -nostdlib "-Wl,--no-entry" "-Wl,--export-all" -o tests/unit/fixtures/test.wasm tests/unit/fixtures/test.c
 */


int add(int a, int b) {
    return a + b;
}

int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int power(int base, int exp) {
    int res = 1;
    for (int i = 0; i < exp; i++) res *= base;
    return res;
}

int fast_power(int base, int exp) {
    int res = 1;
    while (exp > 0) {
        if (exp % 2 == 1) res *= base;
        base *= base;
        exp /= 2;
    }
    return res;
}

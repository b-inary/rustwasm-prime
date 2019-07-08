onmessage = ev => {
  compute(ev.data);
}

async function compute(input: string) {
  const mod = await import('../wasm-pkg/index');

  // Remove extra spaces from input string
  if (/\*\s+\*/.test(input)) {
    postMessage('Parse error: Unexpected character \'*\'');
    return;
  }
  const input_without_spaces = input.replace(/\s*([\+\-\*\/\%\!\(\)])\s*/g, '$1').trim();

  // Load input
  console.time("load input");
  const number_bytes = mod.parse_integer(input_without_spaces);
  console.timeEnd("load input");
  if (number_bytes == null) {
    postMessage(`Parse error: ${mod.parse_error_message(input_without_spaces)}`);
    return;
  }

  // Perform trivial tests
  console.time("trivial tests");
  const result_trivial = mod.trivial_prime_test(number_bytes);
  console.timeEnd("trivial tests");
  if (!result_trivial) {
    postMessage('Result: Not prime');
    return;
  }

  // If the number is small, the following tests are not needed
  if (number_bytes.length <= 2) {
    postMessage('Result: Prime');
    return;
  }

  // Perform a Miller-Rabin test
  console.time("Miller-Rabin test");
  const result_miller_rabin = mod.miller_rabin_test(number_bytes);
  console.timeEnd("Miller-Rabin test");
  if (!result_miller_rabin) {
    postMessage('Result: Not prime');
    return;
  }

  // Perform an extra strong Lucas test
  console.time("Lucas test");
  const result_lucas = mod.extra_strong_lucas_test(number_bytes);
  console.timeEnd("Lucas test");
  if (!result_lucas) {
    postMessage('Result: Not prime');
    return;
  }

  // Hurray! The number is (probably) prime!
  postMessage('Result: Prime');
}

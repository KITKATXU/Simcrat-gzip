# simcrat-gpt4o

[GitHub - kaist-plrg/simcrat](https://github.com/kaist-plrg/simcrat): *Type-Migrating C-to-Rust Translation using a Large Language Model*

- **gzip-1.12**: 
  - 6383 LOC
  - 41 Types
  - 151 Variables
  - 220 Functions
  - 1144 Calls

#### Method

1. Let LLM generate a specified number of candidate function signatures.
2. Provide the function signatures to the caller function for translation.
3. Let LLM compare the translation quality of different function definitions.

#### Experimental Attempts

- Tried gpt-3.5-turbo:
  - 884 previous errors; 86 warnings
- Tried gpt-4o:
  - 487 previous errors; 34 warnings
  - request_tokens: 541042
  - response_tokens: 916480
  - response_time: 28104.332 ms

#### Translation Results

- Empty main function.
- Unsafe code blocks.
- Translated types, variables, and functions one by one, with the output also following this order.
- Very organized and appearing to have no omissions in any of the aforementioned definitions.
# Developer notes etc
List:
- [Project Infrastructure](#project-infrastructure)
- [Actual info](#actual-info)
- [Communication](#communication)


## Project Infrastructure
We are using classic `branch->dev->release` scheme. Every *feature* or *core-component* is developed inside it's branch. On done: Pull request it into `dev` branch of the main remote repository. `dev->release` PR is performed ONLY when version of product is READY AND TESTED.

### Features
Feature is a small(or not) addition to working part of product. For example: we have a calculator.
- **Support** of complex numbers is a `feature`.
- **Libarary** for processing lines and refactoring is a `core_component`.

Every feature branch should be named: `ft_{feature name}`. When you are creating PR into `dev`, describe the feature's main concept and reason you are adding it. Then describe the current state of the feature(e.g. WIP, done etc). Consider testing your feature before creating Pull Request.

### Core-components
Core component is a big addition that changes the project behaviour. For example: use of libraries, tests, deployment things, everything until first release.
Core branches should be named: `core_{componenet name}`. Core components also should be tested in separate with entire project. When creating PR, attach something(test-data, mathematical proof, ...) that proves that component works on itself and ready to be merged with other parts of the project. Also, if core component is a library, binary, test script(s) or anything else, that should be compiled/built on it's own rules, it should be located in it's own subdirectory of the repository.

### Building and full testing
Building full artifacts and testing it happens in main reposistory. It is a job of a main repository owners to ship everything together and test it.

### CI-CD
CD tools are not going to be implemented for this project. CIs are planned to implement when visible progress arrives.

## Actual info
We are working on bmp watermark solution. \
Languages allowed: 
- C
- C++
- Rust
- TypeScript/JavaScript
- Python(Only for tests, scripts and deployment tools)
- Lua(Scripts and configs only) 

We **need**:
- `Cli part`. That will parse command line, check for input errors and will let user know if something went wrong.
- `Static lib`. Library for parsing `BMP` images, providing tools for modifying, resizing, merging, ... This one should be a static library, that could be implemented with C/C++ or Rust.
- `Processing part`. Bridge between parsed from `cli` request and modifying functions from `lib`.

We **might have**:
- `Normal GUI`. If we are not lazzy, we might implement GUI in any way.
- `Another image format supports`. We are not doing this... But maaaaaaaybe
- `Automatic deployment`. Yes, originally, CD tools are not planned. But if everything will go smooth, it would be great to have it.

## Communication
For dev2dev communication use Issues with label `Communication and requests` in the main github repository(https://github.com/LeastDepressedDev/cftw).
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.1.0] - 2025-07-04
### :boom: BREAKING CHANGES
- due to [`e89d2c7`](https://github.com/E-Jacques/rs-file-sorter/commit/e89d2c72875940da068f76ad037ea73d0f0a49a6) - change sorting strategy interface *(commit by [@E-Jacques](https://github.com/E-Jacques))*:

  change sorting strategy interface


### :sparkles: New Features
- [`8ab62af`](https://github.com/E-Jacques/rs-file-sorter/commit/8ab62afd0683528bde79aeae2536e25023004a7e) - **cli**: create specific binary for cli *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`4513dab`](https://github.com/E-Jacques/rs-file-sorter/commit/4513dabed5293376838c5de5096a5e8ebb397e35) - **app**: initialize application and create specific binary *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`05b82e4`](https://github.com/E-Jacques/rs-file-sorter/commit/05b82e49761641659e9beb6dd2899e7edd5936d7) - **ui**: create user interface *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`4e77dc1`](https://github.com/E-Jacques/rs-file-sorter/commit/4e77dc1825081184f30990b6dfdb475d9f7e367f) - **ui**: use icon for strategy selection *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`27e6dbb`](https://github.com/E-Jacques/rs-file-sorter/commit/27e6dbb4d62f349b40bc977d7ffe218d2500d4c4) - **ui**: add folder selection button *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`f3b7049`](https://github.com/E-Jacques/rs-file-sorter/commit/f3b7049e8e76d87d69a58fe32f89ac816988e128) - **ui**: use combo box for strategies selection *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`ce692e1`](https://github.com/E-Jacques/rs-file-sorter/commit/ce692e11fdc27eb9da8fb73f0eabe16c036312e3) - **ui**: improve directory input and extract in separate component *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`4b23493`](https://github.com/E-Jacques/rs-file-sorter/commit/4b23493e693ea94e22f4cab457100b048c3672ad) - **ui**: change window size *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`97da584`](https://github.com/E-Jacques/rs-file-sorter/commit/97da584c0b198fbeb5386f9702a674c853b1fe15) - **strategy**: add concat strategy *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`48580fd`](https://github.com/E-Jacques/rs-file-sorter/commit/48580fd76a55acc232ed03393c0d9ec3ab957dc1) - **cli**: add support for linked arguments *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`e24d785`](https://github.com/E-Jacques/rs-file-sorter/commit/e24d785fb1f4e42be5261c6e3accb46886590f4a) - **ui**: add nested editable tree items *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`8f46002`](https://github.com/E-Jacques/rs-file-sorter/commit/8f4600243b758e951ae9e4a1cddf6c1cf8942a7a) - **core**: add parameter support for strategy *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`53d5b36`](https://github.com/E-Jacques/rs-file-sorter/commit/53d5b36e91907eb01e4ff7550550af8c32edf4c4) - **strategy**: handle parameter support *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`764caec`](https://github.com/E-Jacques/rs-file-sorter/commit/764caec87cbce3db4b344c3b8618e7a81bebf5f4) - **ui**: add support for concat *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`3e84e76`](https://github.com/E-Jacques/rs-file-sorter/commit/3e84e7692c3aa9db4e3296f00a85218512dd2fed) - **cli**: add support for concat strategy *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`b035e55`](https://github.com/E-Jacques/rs-file-sorter/commit/b035e5585e5ce549096aefd845b48568d1b199f0) - **strategy**: add catalog system *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`19b429e`](https://github.com/E-Jacques/rs-file-sorter/commit/19b429e7de1c9aa5e5b1bab2aa3f695e5c01cdcb) - **cli**: add catalog support *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`835be9b`](https://github.com/E-Jacques/rs-file-sorter/commit/835be9b378504a9c392ce7c285246e6b380ca20f) - **core**: add SingleString strategy parameter *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`82ae96f`](https://github.com/E-Jacques/rs-file-sorter/commit/82ae96fb09d108f5a78c1e979d6edb51152e03a7) - **strategy**: add text strategy *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`ff35e75`](https://github.com/E-Jacques/rs-file-sorter/commit/ff35e75daf4a39ab75d2af56def66643a4407b43) - **cli**: add support for test strategy *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`923a5c0`](https://github.com/E-Jacques/rs-file-sorter/commit/923a5c07bbd5514a4e147038f3eb6f383fe2eeba) - **core**: add strategy validator system *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`1dbbf90`](https://github.com/E-Jacques/rs-file-sorter/commit/1dbbf90354571e2bb34d8ee572c9d4245e3c3de9) - **strategy**: add validator to strategies *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`bcb6eab`](https://github.com/E-Jacques/rs-file-sorter/commit/bcb6eab88bc6ec6d24b1f3a22122af30b673b992) - **ui**: add support for strategy properties *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`be4e798`](https://github.com/E-Jacques/rs-file-sorter/commit/be4e79834d46182e5bcf7bf953ca1592f9f8b30b) - **cli**: add support for parameters *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`e5cd32c`](https://github.com/E-Jacques/rs-file-sorter/commit/e5cd32ca8e5672c4a679dad7730ea4b91eaca107) - **ui**: improve application ui *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`d0c16cd`](https://github.com/E-Jacques/rs-file-sorter/commit/d0c16cd2d1b7831351eb518ee0031bde42dbcc4f) - **ui**: improve look of input/output directory *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`dc2960d`](https://github.com/E-Jacques/rs-file-sorter/commit/dc2960d1ac41af68a659af9f3129979cc424fcb8) - **core**: add choice strategy parameter kind *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`d54fa46`](https://github.com/E-Jacques/rs-file-sorter/commit/d54fa46ef5358e5f15827e7ac3fef90d9ceb7546) - **cli**: support choice parameter *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`196f1df`](https://github.com/E-Jacques/rs-file-sorter/commit/196f1df1f599542777ee32b9425dc298433315f4) - **ui**: support choice parameters *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`6a2b91f`](https://github.com/E-Jacques/rs-file-sorter/commit/6a2b91ffaa3715499b3a8da85f81f123dd4ae5f6) - **core**: add support for default value *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`86fa6f9`](https://github.com/E-Jacques/rs-file-sorter/commit/86fa6f96460ae2be4116a9b8962d40d4309d6854) - **strategy**: add locale parameter to month *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`309846f`](https://github.com/E-Jacques/rs-file-sorter/commit/309846fa27bd1055078b54234a83a22ec8e167f8) - **ui**: add support for default_value *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`818f6b5`](https://github.com/E-Jacques/rs-file-sorter/commit/818f6b586d97dd4c67dd60703a54f703f272c5e1) - **core**: improve error handling *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`296a204`](https://github.com/E-Jacques/rs-file-sorter/commit/296a204a489a8bd79a98d0653d522ae837021627) - **ui**: support new error system *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`e747fb8`](https://github.com/E-Jacques/rs-file-sorter/commit/e747fb89584da9b0ad30164531e781fed29ae24d) - add support for dry run *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`91a49b3`](https://github.com/E-Jacques/rs-file-sorter/commit/91a49b3abc015ef0777fd221be840b7f3994daf0) - **ui**: improve preview tree look & feel *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`40f5da9`](https://github.com/E-Jacques/rs-file-sorter/commit/40f5da959a06a24fbdd0b2bcaeedf41b7167ad5a) - add support for only root level option *(commit by [@E-Jacques](https://github.com/E-Jacques))*

### :bug: Bug Fixes
- [`c6ac5fa`](https://github.com/E-Jacques/rs-file-sorter/commit/c6ac5fa4593be8a8d1e11fcb1382dec55fd9c536) - **cli**: remove typo in help descriptions *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`5ab79ef`](https://github.com/E-Jacques/rs-file-sorter/commit/5ab79ef1a4822e3bce36d72de3b2c97817be8acd) - **ui**: clear parameters when strategy change *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`cc48225`](https://github.com/E-Jacques/rs-file-sorter/commit/cc48225454e01760185ac6911c76b7d5c3f70a9c) - add file filter to root only getter *(commit by [@E-Jacques](https://github.com/E-Jacques))*

### :recycle: Refactors
- [`8550ecc`](https://github.com/E-Jacques/rs-file-sorter/commit/8550ecc988118dae6d90060a39b4fa391a122b48) - fix package typo 'sorterer-stretegies' -> 'sorting_strategies' *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`c3f5aa3`](https://github.com/E-Jacques/rs-file-sorter/commit/c3f5aa3e2d70f1abc0a7b5436fe56177c9cda246) - remove redundant call *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`9787023`](https://github.com/E-Jacques/rs-file-sorter/commit/97870232baecb8a783b041083b92e2f0ca8236a9) - move cli handler to cli package *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`e89d2c7`](https://github.com/E-Jacques/rs-file-sorter/commit/e89d2c72875940da068f76ad037ea73d0f0a49a6) - change sorting strategy interface *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`3fa4822`](https://github.com/E-Jacques/rs-file-sorter/commit/3fa48228c8cc7c405661e3019809f82cc96dcca8) - **strategy**: use new static function to create strategies *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`283d3f1`](https://github.com/E-Jacques/rs-file-sorter/commit/283d3f1fc6d31b20204d982cf8b7d5d860122d04) - **cli**: simplify parser at cost of performance *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`8870e46`](https://github.com/E-Jacques/rs-file-sorter/commit/8870e46cf17f83c581956a7f5ad80e7951f17bb0) - **ui**: deconstruct & rename editable file tree *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`0e0272a`](https://github.com/E-Jacques/rs-file-sorter/commit/0e0272af0cf0f81fef9a5d5c8c4a218dabcc87db) - **ui**: use trait for editable tree item *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`676ef85`](https://github.com/E-Jacques/rs-file-sorter/commit/676ef8533c5f0648cb90d0102c1ae79f46c65d67) - **ui**: add support for catalog *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`cb3df29`](https://github.com/E-Jacques/rs-file-sorter/commit/cb3df2964508827693e6aea0e7b8c28401198d50) - extract strategy param to new file *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`01142af`](https://github.com/E-Jacques/rs-file-sorter/commit/01142af8d4292d33d6bec9460007e0c6311f6624) - **cli**: improve the way error are handled *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`0bc1b8d`](https://github.com/E-Jacques/rs-file-sorter/commit/0bc1b8d5d6159b4ef79aadb5563f796986cab1c9) - **strategy**: improve code readability *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`d9777a0`](https://github.com/E-Jacques/rs-file-sorter/commit/d9777a0275aee537b557737cb9ef29530e0b074b) - **cli**: add support for new error system *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`b49b151`](https://github.com/E-Jacques/rs-file-sorter/commit/b49b1516d753e7f7036f49346dd51da4d46c7d66) - **ui**: reorganize screen structure *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`94f5782`](https://github.com/E-Jacques/rs-file-sorter/commit/94f57825e317c3ea23c76dbfc556b3a174dac573) - **ui**: extract pipeline processing *(commit by [@E-Jacques](https://github.com/E-Jacques))*

### :white_check_mark: Tests
- [`c10a7bd`](https://github.com/E-Jacques/rs-file-sorter/commit/c10a7bdcca3e3b9fd2d3a8cd7e3211f2e6874cf1) - clean up e2e testing *(commit by [@E-Jacques](https://github.com/E-Jacques))*

### :wrench: Chores
- [`b7ad0a8`](https://github.com/E-Jacques/rs-file-sorter/commit/b7ad0a871970447a8fdcdb05a3f93c59c81f7db9) - **cargo**: remove test property from cargo package *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`1070821`](https://github.com/E-Jacques/rs-file-sorter/commit/10708211c46abed5ccb516d12c0a5b323e9183d4) - move core element to specific module *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`16f6875`](https://github.com/E-Jacques/rs-file-sorter/commit/16f6875a7cc8cf3696f32ca6d75bddf9c20b0185) - delegate sorting strategy list to user *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`1144b5a`](https://github.com/E-Jacques/rs-file-sorter/commit/1144b5a172efb3255a6b23dbdcd7af1dad3afb5c) - simplify sorting strategy structure *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`305d92e`](https://github.com/E-Jacques/rs-file-sorter/commit/305d92edc77fe9c1c8f068c597f588e6b0fc4ac8) - move cli relative module to cli module as submodules *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`5ebb3b8`](https://github.com/E-Jacques/rs-file-sorter/commit/5ebb3b80d3dad38a75ed58bb5b77002b4534e8e2) - add random string gen *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`96ee294`](https://github.com/E-Jacques/rs-file-sorter/commit/96ee294737de1233c45098e4e7729a498f20332e) - **deps**: add rfd dependency *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`5f00c2e`](https://github.com/E-Jacques/rs-file-sorter/commit/5f00c2efd3a6d23c89b1dea7be0731d9b1f399ae) - add entry in gitignore *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`bb85ad4`](https://github.com/E-Jacques/rs-file-sorter/commit/bb85ad405e8a9bc51de15393de1654aca7ec087b) - **core**: remove println *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`9291d08`](https://github.com/E-Jacques/rs-file-sorter/commit/9291d08072449c239ed2c0cfe0794f36365268cd) - remove some debug println *(commit by [@E-Jacques](https://github.com/E-Jacques))*
- [`5ce3da7`](https://github.com/E-Jacques/rs-file-sorter/commit/5ce3da7737ce0733c4b04147eff2c2dd732700eb) - remove unused function. *(commit by [@E-Jacques](https://github.com/E-Jacques))*

[v1.1.0]: https://github.com/E-Jacques/rs-file-sorter/compare/1.0.0...v1.1.0

# [2.0.0](https://github.com/DASPRiD/aw2013/compare/v1.0.1...v2.0.0) (2024-01-08)


### Features

* replace rppal dependency with embedded-hal 1.x ([4fc60f7](https://github.com/DASPRiD/aw2013/commit/4fc60f7c6c8cf32a05c82e65b8291115ef76d737)), closes [#1](https://github.com/DASPRiD/aw2013/issues/1)


### BREAKING CHANGES

* You now need to supply your own i2c adapter, either from rppal
or from linux-embedded-hal.

## [1.0.1](https://github.com/DASPRiD/aw2013/compare/v1.0.0...v1.0.1) (2023-12-10)


### Bug Fixes

* **deps:** bump rppal up to 0.16.0 to address hardware detection issue ([c18f096](https://github.com/DASPRiD/aw2013/commit/c18f096e795afe1b0b5326be15cda889b28a91ea))

# 1.0.0 (2023-01-26)


### Features

* create initial implementation ([ca3af03](https://github.com/DASPRiD/aw2013/commit/ca3af03eb75037bc799d7f35892473b792e33e2a))

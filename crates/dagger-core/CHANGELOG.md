# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.11 (2023-04-29)

### New Features

 - <csr-id-2a29a66217fa4d6c530ea1ce670c8836383e7051/> dagger-run support
 - <csr-id-eb7470c604169d1a15976078c0889d5cc7011257/> update to dagger-5.1

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - dagger-run support ([`2a29a66`](https://github.com/kjuulh/dagger-sdk/commit/2a29a66217fa4d6c530ea1ce670c8836383e7051))
    - update to dagger-5.1 ([`eb7470c`](https://github.com/kjuulh/dagger-sdk/commit/eb7470c604169d1a15976078c0889d5cc7011257))
</details>

## v0.2.10 (2023-04-25)

### Bug Fixes

 - <csr-id-9d3c21d16b4a64eb7a7b1888365a4c4ea56d7225/> delete other files/folder in downloads: #57

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 21 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.10 ([`8011c42`](https://github.com/kjuulh/dagger-sdk/commit/8011c42dc077d101b1bccaf231fac17636dd249d))
    - delete other files/folder in downloads: #57 ([`9d3c21d`](https://github.com/kjuulh/dagger-sdk/commit/9d3c21d16b4a64eb7a7b1888365a4c4ea56d7225))
</details>

## v0.2.9 (2023-04-03)

<csr-id-b55bcc159ffc6a61ecfcc5e3aa3de00a1a73b5b8/>

### New Features

 - <csr-id-114f411cdb0e1043071c0ccc1768d344f78d4fcb/> with 0.4.2
 - <csr-id-b094ae4f539a880b0bde12841b7db1fbfcc0f123/> add musl ci
   This pr adds musl ci support for #50
 - <csr-id-2faabb0e502a9b15c88b0bdf5673d1b458198d70/> with multi platform ci
   This adds the first iteration of multi platform ci. This is the lowest level of testing added: pinned all the way to nightly. Next up will be macos, then musl and arm. And lastly windows. Each will probably require special handling, especially because of how cross and qemu interacts with the dagger-engine and docker.
 - <csr-id-11d20935c697e28caaa671e8da0e70a99d4310fc/> extract client
   This extracts the client (strategy pattern), this is so that it is will be possible to test the actual querier, without hitting / requiring the dagger-engine running.
 - <csr-id-384294b39038123b02c406a1038105b111c3b9be/> rename projects to point to github/kjuulh/dagger-sdk
 - <csr-id-79d931e908c58a0464fd9cf7d6ef02eb50f14c23/> with loggers

### Refactor

 - <csr-id-b55bcc159ffc6a61ecfcc5e3aa3de00a1a73b5b8/> move dagger-rs and adopt workspace.deps

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 19 calendar days.
 - 23 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#46](https://github.com/kjuulh/dagger-sdk/issues/46), [#48](https://github.com/kjuulh/dagger-sdk/issues/48), [#51](https://github.com/kjuulh/dagger-sdk/issues/51)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#46](https://github.com/kjuulh/dagger-sdk/issues/46)**
    - with multi platform ci ([`2faabb0`](https://github.com/kjuulh/dagger-sdk/commit/2faabb0e502a9b15c88b0bdf5673d1b458198d70))
 * **[#48](https://github.com/kjuulh/dagger-sdk/issues/48)**
    - extract client ([`11d2093`](https://github.com/kjuulh/dagger-sdk/commit/11d20935c697e28caaa671e8da0e70a99d4310fc))
 * **[#51](https://github.com/kjuulh/dagger-sdk/issues/51)**
    - add musl ci ([`b094ae4`](https://github.com/kjuulh/dagger-sdk/commit/b094ae4f539a880b0bde12841b7db1fbfcc0f123))
 * **Uncategorized**
    - Release dagger-core v0.2.9, dagger-sdk v0.2.20 ([`f82075c`](https://github.com/kjuulh/dagger-sdk/commit/f82075c23808073d9500df63c1cd347cd9b99cef))
    - with 0.4.2 ([`114f411`](https://github.com/kjuulh/dagger-sdk/commit/114f411cdb0e1043071c0ccc1768d344f78d4fcb))
    - rename projects to point to github/kjuulh/dagger-sdk ([`384294b`](https://github.com/kjuulh/dagger-sdk/commit/384294b39038123b02c406a1038105b111c3b9be))
    - with loggers ([`79d931e`](https://github.com/kjuulh/dagger-sdk/commit/79d931e908c58a0464fd9cf7d6ef02eb50f14c23))
    - move dagger-rs and adopt workspace.deps ([`b55bcc1`](https://github.com/kjuulh/dagger-sdk/commit/b55bcc159ffc6a61ecfcc5e3aa3de00a1a73b5b8))
</details>

## v0.2.8 (2023-03-10)

### New Features

 - <csr-id-41b20b6268db9d8defe333694e4d3ec019d7c923/> bump version
 - <csr-id-5f9b3a19c0ab6988bc335b020052074f3f101305/> set internal warnings as errors
 - <csr-id-f9e7af931d94fbedacf74f5da9a2f71b1992324b/> introduce tests again

### Bug Fixes

 - <csr-id-ecca036bc644fee93fbcb69bf6da9f29169e473e/> fix builder pattern to actually work with default values
   In previous versions the builder pattern required all values to be set.
   This has not been fixed, so that default values are allowed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.8, dagger-sdk v0.2.16 ([`f390eac`](https://github.com/kjuulh/dagger-sdk/commit/f390eac29f1d041d18d2207a5aa0a8d993aab68c))
    - bump version ([`41b20b6`](https://github.com/kjuulh/dagger-sdk/commit/41b20b6268db9d8defe333694e4d3ec019d7c923))
    - set internal warnings as errors ([`5f9b3a1`](https://github.com/kjuulh/dagger-sdk/commit/5f9b3a19c0ab6988bc335b020052074f3f101305))
    - introduce tests again ([`f9e7af9`](https://github.com/kjuulh/dagger-sdk/commit/f9e7af931d94fbedacf74f5da9a2f71b1992324b))
    - fix builder pattern to actually work with default values ([`ecca036`](https://github.com/kjuulh/dagger-sdk/commit/ecca036bc644fee93fbcb69bf6da9f29169e473e))
</details>

## v0.2.7 (2023-02-24)

### New Features

 - <csr-id-3e8ca8d86eafdc1f9d5e8b69f14fb60509549e0f/> update to dagger-v0.3.13

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.7, dagger-sdk v0.2.15 ([`6a9a560`](https://github.com/kjuulh/dagger-sdk/commit/6a9a560cdca097abf23371d44599a2f1b726ae7f))
    - update to dagger-v0.3.13 ([`3e8ca8d`](https://github.com/kjuulh/dagger-sdk/commit/3e8ca8d86eafdc1f9d5e8b69f14fb60509549e0f))
</details>

## v0.2.6 (2023-02-20)

<csr-id-1f77d90c0f0ac832a181b2322350ea395612986c/>

### Chore

 - <csr-id-1f77d90c0f0ac832a181b2322350ea395612986c/> ran clippy

### Bug Fixes

 - <csr-id-8dfecf976c5537cc2c03881de2b2fd2b2508683a/> cli session keep session alive

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.6, dagger-codegen v0.2.7, dagger-sdk v0.2.12 ([`7179f8b`](https://github.com/kjuulh/dagger-sdk/commit/7179f8b598ef04e62925e39d3f55740253c01686))
    - ran clippy ([`1f77d90`](https://github.com/kjuulh/dagger-sdk/commit/1f77d90c0f0ac832a181b2322350ea395612986c))
    - cli session keep session alive ([`8dfecf9`](https://github.com/kjuulh/dagger-sdk/commit/8dfecf976c5537cc2c03881de2b2fd2b2508683a))
</details>

## v0.2.5 (2023-02-20)

### Bug Fixes

 - <csr-id-a13a2a9ecbfdfac80ed8eb0cbb9e9db317da65de/> race condition in process

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.5, dagger-sdk v0.2.12, dagger-codegen v0.2.7 ([`1725c51`](https://github.com/kjuulh/dagger-sdk/commit/1725c5188e8a81069ec4a4de569484c921a94927))
    - race condition in process ([`a13a2a9`](https://github.com/kjuulh/dagger-sdk/commit/a13a2a9ecbfdfac80ed8eb0cbb9e9db317da65de))
</details>

## v0.2.4 (2023-02-20)

### Bug Fixes

 - <csr-id-8385aa8a15ff7b45fecc3462c482b998118c14eb/> remove blocking
 - <csr-id-921e61b5e248013cb5fbf4f1bad3eef5a2673145/> remove blocking

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.4, dagger-codegen v0.2.6, dagger-sdk v0.2.11 ([`f869e57`](https://github.com/kjuulh/dagger-sdk/commit/f869e574dd788cd60e5b1b5d502bec68e300694c))
    - remove blocking ([`921e61b`](https://github.com/kjuulh/dagger-sdk/commit/921e61b5e248013cb5fbf4f1bad3eef5a2673145))
    - Release dagger-core v0.2.4, dagger-codegen v0.2.6, dagger-sdk v0.2.11 ([`17ec62a`](https://github.com/kjuulh/dagger-sdk/commit/17ec62a5d58232ff57391523b9851fb7b07d02ab))
    - remove blocking ([`8385aa8`](https://github.com/kjuulh/dagger-sdk/commit/8385aa8a15ff7b45fecc3462c482b998118c14eb))
</details>

## v0.2.3 (2023-02-20)

### Bug Fixes

 - <csr-id-75bc17e57db222492c6ffd2dfe80208d2bda75c9/> Fix async panic on blocking #19
   Replaced internal threads with tokio spawn functions

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.3, dagger-sdk v0.2.9, dagger-rs v0.2.10 ([`82de43a`](https://github.com/kjuulh/dagger-sdk/commit/82de43aa91d6a5e09a247d1959137fdc36a40d35))
    - Fix async panic on blocking #19 ([`75bc17e`](https://github.com/kjuulh/dagger-sdk/commit/75bc17e57db222492c6ffd2dfe80208d2bda75c9))
</details>

## v0.2.2 (2023-02-19)

### New Features

 - <csr-id-6e5f4074329ab0462445b31d4153f8497c483438/> update to dagger v0.3.12

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release dagger-core v0.2.2, dagger-codegen v0.2.2, dagger-rs v0.2.8 ([`1638f15`](https://github.com/kjuulh/dagger-sdk/commit/1638f15fba9d16512e8452f87b908d6dce417cd9))
    - update to dagger v0.3.12 ([`6e5f407`](https://github.com/kjuulh/dagger-sdk/commit/6e5f4074329ab0462445b31d4153f8497c483438))
</details>

## v0.2.1 (2023-02-18)

### Bug Fixes

 - <csr-id-789b0e69c8c53d0e86d9cec89ab5345507aad514/> update all dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 19 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#5](https://github.com/kjuulh/dagger-sdk/issues/5), [#6](https://github.com/kjuulh/dagger-sdk/issues/6)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#5](https://github.com/kjuulh/dagger-sdk/issues/5)**
    - update all dependencies ([`789b0e6`](https://github.com/kjuulh/dagger-sdk/commit/789b0e69c8c53d0e86d9cec89ab5345507aad514))
 * **[#6](https://github.com/kjuulh/dagger-sdk/issues/6)**
    - feature/add impl ([`4a4c03f`](https://github.com/kjuulh/dagger-sdk/commit/4a4c03f3c2ee7f6268c65976715e70767b4ea78d))
 * **Uncategorized**
    - Release dagger-core v0.2.1, dagger-codegen v0.2.1, dagger-rs v0.2.1 ([`1332bc8`](https://github.com/kjuulh/dagger-sdk/commit/1332bc842ce2ea0254c651419813b63b36ca590c))
    - add changelogs ([`a064684`](https://github.com/kjuulh/dagger-sdk/commit/a064684fcf80196188a57d9ff9067c0b5769fb09))
    - Adjusting changelogs prior to release of dagger-core v0.2.1, dagger-codegen v0.2.1, dagger-rs v0.2.1 ([`f4a20fd`](https://github.com/kjuulh/dagger-sdk/commit/f4a20fda79063b29829cc899793775ba8cb17214))
    - with publish ([`989d5bc`](https://github.com/kjuulh/dagger-sdk/commit/989d5bc26036d46a199d939b5cbbe72aff2f8fb1))
    - with repo ([`e5383b5`](https://github.com/kjuulh/dagger-sdk/commit/e5383b51f3b290a87b729929c377e93bda3873e0))
    - with readme and license ([`1e26b38`](https://github.com/kjuulh/dagger-sdk/commit/1e26b383d4f6dbcbe20f5f7c19c749e743f6e607))
    - bump version ([`36b0ecd`](https://github.com/kjuulh/dagger-sdk/commit/36b0ecdabf4c220cffb2d0660fb6480387e3249a))
    - with println ([`d1726a0`](https://github.com/kjuulh/dagger-sdk/commit/d1726a052a6dc4e57f364864446cab3cbda7e0bf))
    - context and querier done ([`52a0db3`](https://github.com/kjuulh/dagger-sdk/commit/52a0db3e311f9f88447882f1406701d4cd612b1c))
    - tested full flow initially ([`7a008be`](https://github.com/kjuulh/dagger-sdk/commit/7a008be59e5ca183809e5840cdfae1d87665aa20))
    - move code to dagger-core ([`ec0d0b2`](https://github.com/kjuulh/dagger-sdk/commit/ec0d0b22e646c97acb3ce93f3afb3ddb8590e68f))
    - add fields ([`496a687`](https://github.com/kjuulh/dagger-sdk/commit/496a687bc34f7c58cc86df60c183be741b0b8a9c))
    - with objects ([`5fef514`](https://github.com/kjuulh/dagger-sdk/commit/5fef5148010f384d0158361d64b8e17d357d4819))
    - with enum ([`2a1f7c3`](https://github.com/kjuulh/dagger-sdk/commit/2a1f7c3f2666f1f4caebf7c22707709741c2cfad))
</details>


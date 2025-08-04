# MatchingWildcardsInRust
Matching Wildcards in Rust

This file set includes ASCII and UTF-8-ready routines for matching wildcards in Rust (fast_wild_compare.rs), ported from the native code implementation here: http://www.developforperformance.com/MatchingWildcards_AnImprovedAlgorithmForBigData.html

It also includes Rust implementations of ASCII testscases for correctness and performance, originally implemented in C/C++, plus a new set of UTF-8 testcases.

A description of the algorithm's history, implementation and testing strategies, performance findings, and thoughts about how to choose one routine over another appear here: http://www.developforperformance.com/MatchingWildcardsInRust.html

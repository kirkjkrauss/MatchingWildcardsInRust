// Rust testcases for matching wildcards.
//
// Copyright 2025 Kirk J Krauss.  This is a Derivative Work based on 
// material that is copyright 2018 IBM Corporation and available at
//
//  http://developforperformance.com/MatchingWildcards_AnImprovedAlgorithmForBigData.html
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// This file provides Rust testcases for matching wildcards, including sets 
// of correctness and performance tests, along with a main() routine that 
// invokes the testcases and outputs the results.

// Testcase selection flags.
const COMPARE_PERFORMANCE: bool = true;
const COMPARE_WILD: bool = true;
const COMPARE_TAME: bool = true;
const COMPARE_EMPTY: bool = true;
const TEST_UTF8: bool = false;

// File=scope variables for accumulating performance data.
static mut U_RUST_TIME_ASCII: u128 = 0;
static mut U_RUST_TIME_UTF8: u128 = 0;
static mut U_CPP_TIME_FASTEST: u128 = 0;
static mut U_CPP_TIME_PORTABLE: u128 = 0;

// Standard modules for use with the String type, C/C++ functions, and 
// performance tests.
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Instant;

// Declarations for ASCII and UTF-8 functions for matching wildcards in Rust.
mod fast_wild_compare;

// Declarations for performance comparison with C++ versions of the algorithm
// on which the ASCII and UTF-8 functions are baseed.
unsafe extern "C" {
    pub fn FastWildCompare(
        ptame: *mut cty::c_char,
        pwild: *mut cty::c_char,
    ) -> bool;

    pub fn FastWildComparePortable(
        ptame: *mut cty::c_char,
        pwild: *mut cty::c_char,
    ) -> bool;
}


// This function compares a tame/wild string pair via each included routine.
//
fn test(tame_string: String, wild_string: String, 
        b_expected_result: bool) -> bool
{
	if COMPARE_PERFORMANCE
	{
		// Get execution times for our two Rust routines.
		let timer_1 = Instant::now();

		if b_expected_result != fast_wild_compare::fast_wild_compare_ascii(
			&wild_string, &tame_string)
		{
			return false;
		}

		unsafe  // For the sake of avoiding piles of passed parameters.
		{
			U_RUST_TIME_ASCII += timer_1.elapsed().as_nanos();
		}

		let timer_2 = Instant::now();

		// Allocate array-style memory and initialize with each input String's 
		// 32-bit UTF-8 code points.
		//
		// A memory allocation failure can be associated with a panic.  In a 
		// situation involving many calls to this routine, arrangements to 
		// catch allocation failures may be placed around that entire set of 
		// calls.
		//
		if b_expected_result != fast_wild_compare::fast_wild_compare_utf8(
		        wild_string.chars().collect::<Vec<char>>().into_boxed_slice(),
		        tame_string.chars().collect::<Vec<char>>().into_boxed_slice())
		{
			return false;
		}

		unsafe
		{
			U_RUST_TIME_UTF8 += timer_2.elapsed().as_nanos();
		}

		// For comparison, get execution times for the C/C++ versions.
		unsafe
		{
			let c_wild = CString::new(wild_string).expect(
			                          "CString::new failed");
			let c_tame = CString::new(tame_string).expect(
			                          "CString::new failed");
			let c_wild_ptr: *mut c_char = c_wild.into_raw();
			let c_tame_ptr: *mut c_char = c_tame.into_raw();

			let timer_3 = Instant::now();

			if b_expected_result != FastWildCompare(
			       c_wild_ptr, c_tame_ptr)
			{
				return false;
			}

			U_CPP_TIME_FASTEST += timer_3.elapsed().as_nanos();

			let timer_4 = Instant::now();

			if b_expected_result != FastWildComparePortable(
			       c_wild_ptr, c_tame_ptr)
			{
				return false;
			}

			U_CPP_TIME_PORTABLE += timer_4.elapsed().as_nanos();
		}
	}
	else if TEST_UTF8
	{
		// Case-insensitive matching:
		// Allocate array-style memory and initialize with each input String's 
		// lowercased 32-bit UTF-8 code points.
		//
		// A memory allocation failure can be associated with a panic.  See 
		// above comment regarding catching that situation in production code.
		//
	    if b_expected_result != fast_wild_compare::fast_wild_compare_utf8(
			       wild_string.to_lowercase(
		                  ).chars().collect::<Vec<char>>().into_boxed_slice(),
		           tame_string.to_lowercase(
		                  ).chars().collect::<Vec<char>>().into_boxed_slice())
		{
			return false;
		}
	}
	else if b_expected_result != fast_wild_compare::fast_wild_compare_ascii(&wild_string, 
	            &tame_string)
	{
		return false;
	}

	return true;
}


// A set of wildcard comparison tests.
//
fn test_wild()
{
    let mut i_reps: i32;
    let mut b_all_passed: bool = true;

	if COMPARE_PERFORMANCE
	{
		// Can choose as many repetitions as you might expect in production.
		i_reps = 1;
	}
	else
	{
		i_reps = 1;
	}

    while i_reps > 0
    {
		i_reps -= 1;
		
		// Case with first wildcard after total match.
        b_all_passed &= test("Hi".into(), "Hi*".into(), true);
		
		// Case with mismatch after '*'.
        b_all_passed &= test("abc".into(), "ab*d".into(), false);

        // Cases with repeating character sequences.
        b_all_passed &= test("abcccd".into(), "*ccd".into(), true);
        b_all_passed &= test("mississipissippi".into(), "*issip*ss*".into(), true);
        b_all_passed &= test("xxxx*zzzzzzzzy*f".into(), "xxxx*zzy*fffff".into(), false);
        b_all_passed &= test("xxxx*zzzzzzzzy*f".into(), "xxx*zzy*f".into(), true);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "xxxx*zzy*fffff".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "xxxx*zzy*f".into(), true);
        b_all_passed &= test("xyxyxyzyxyz".into(), "xy*z*xyz".into(), true);
        b_all_passed &= test("mississippi".into(), "*sip*".into(), true);
        b_all_passed &= test("xyxyxyxyz".into(), "xy*xyz".into(), true);
        b_all_passed &= test("mississippi".into(), "mi*sip*".into(), true);
        b_all_passed &= test("ababac".into(), "*abac*".into(), true);
        b_all_passed &= test("ababac".into(), "*abac*".into(), true);
        b_all_passed &= test("aaazz".into(), "a*zz*".into(), true);
        b_all_passed &= test("a12b12".into(), "*12*23".into(), false);
        b_all_passed &= test("a12b12".into(), "a12b".into(), false);
        b_all_passed &= test("a12b12".into(), "*12*12*".into(), true);

		if COMPARE_PERFORMANCE
		{
			// From DDJ reader Andy Belf.
			b_all_passed &= test("caaab".into(), "*a?b".into(), true);
		}

        // Additional cases where the '*' char appears in the tame string.
        b_all_passed &= test("*".into(), "*".into(), true);
        b_all_passed &= test("a*abab".into(), "a*b".into(), true);
        b_all_passed &= test("a*r".into(), "a*".into(), true);
        b_all_passed &= test("a*ar".into(), "a*aar".into(), false);

        // More double wildcard scenarios.
        b_all_passed &= test("XYXYXYZYXYz".into(), "XY*Z*XYz".into(), true);
        b_all_passed &= test("missisSIPpi".into(), "*SIP*".into(), true);
        b_all_passed &= test("mississipPI".into(), "*issip*PI".into(), true);
        b_all_passed &= test("xyxyxyxyz".into(), "xy*xyz".into(), true);
        b_all_passed &= test("miSsissippi".into(), "mi*sip*".into(), true);
        b_all_passed &= test("miSsissippi".into(), "mi*Sip*".into(), false);
        b_all_passed &= test("abAbac".into(), "*Abac*".into(), true);
        b_all_passed &= test("abAbac".into(), "*Abac*".into(), true);
        b_all_passed &= test("aAazz".into(), "a*zz*".into(), true);
        b_all_passed &= test("A12b12".into(), "*12*23".into(), false);
        b_all_passed &= test("a12B12".into(), "*12*12*".into(), true);
        b_all_passed &= test("oWn".into(), "*oWn*".into(), true);

        // Completely tame (no wildcards) cases.
        b_all_passed &= test("bLah".into(), "bLah".into(), true);
        b_all_passed &= test("bLah".into(), "bLaH".into(), false);

        // Simple mixed wildcard tests suggested by Marlin Deckert.
        b_all_passed &= test("a".into(), "*?".into(), true);
        b_all_passed &= test("ab".into(), "*?".into(), true);
        b_all_passed &= test("abc".into(), "*?".into(), true);

        // More mixed wildcard tests including coverage for false positives.
        b_all_passed &= test("a".into(), "??".into(), false);
        b_all_passed &= test("ab".into(), "?*?".into(), true);
        b_all_passed &= test("ab".into(), "*?*?*".into(), true);
        b_all_passed &= test("abc".into(), "?**?*?".into(), true);
        b_all_passed &= test("abc".into(), "?**?*&?".into(), false);
        b_all_passed &= test("abcd".into(), "?b*??".into(), true);
        b_all_passed &= test("abcd".into(), "?a*??".into(), false);
        b_all_passed &= test("abcd".into(), "?**?c?".into(), true);
        b_all_passed &= test("abcd".into(), "?**?d?".into(), false);
        b_all_passed &= test("abcde".into(), "?*b*?*d*?".into(), true);

        // Single-character-match cases.
        b_all_passed &= test("bLah".into(), "bL?h".into(), true);
        b_all_passed &= test("bLaaa".into(), "bLa?".into(), false);
        b_all_passed &= test("bLah".into(), "bLa?".into(), true);
        b_all_passed &= test("bLaH".into(), "?Lah".into(), false);
        b_all_passed &= test("bLaH".into(), "?LaH".into(), true);

        // Many-wildcard scenarios.
        b_all_passed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".into(), 
            "a*a*a*a*a*a*aa*aaa*a*a*b".into(), true);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "*a*b*ba*ca*a*aa*aaa*fa*ga*b*".into(), true);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "*a*b*ba*ca*a*x*aaa*fa*ga*b*".into(), false);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "*a*b*ba*ca*aaaa*fa*ga*gggg*b*".into(), false);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "*a*b*ba*ca*aaaa*fa*ga*ggg*b*".into(), true);
        b_all_passed &= test("aaabbaabbaab".into(), "*aabbaa*a*".into(), true);
        b_all_passed &= test("a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".into(), 
            "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".into(), true);
        b_all_passed &= test("aaaaaaaaaaaaaaaaa".into(), 
            "*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".into(), true);
        b_all_passed &= test("aaaaaaaaaaaaaaaa".into(), 
            "*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".into(), false);
        b_all_passed &= test("abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*a\
bcdefghij*abcdefghijk*abcdefghijkl*abcdefghijklm*abcdefghijklmn".into(), 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*a\
            bc*".into(), false);
        b_all_passed &= test("abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*a\
bcdefghij*abcdefghijk*abcdefghijkl*abcdefghijklm*abcdefghijklmn".into(), 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*".into(), true);
        b_all_passed &= test("abc*abcd*abcd*abc*abcd".into(), 
		    "abc*abc*abc*abc*abc".into(), false);
        b_all_passed &= test(
            "abc*abcd*abcd*abc*abcd*abcd*abc*abcd*abc*abc*abcd".into(), 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abcd".into(), true);
        b_all_passed &= test("abc".into(), 
		    "********a********b********c********".into(), true);
        b_all_passed &= test("********a********b********c********".into(), 
		    "abc".into(), false);
        b_all_passed &= test("abc".into(), 
		    "********a********b********b********".into(), false);
        b_all_passed &= test("*abc*".into(), "***a*b*c***".into(), true);

        // A case-insensitive algorithm test.
        // b_all_passed &= test("mississippi".into(), "*issip*PI".into(), true);

        // Tests suggested by other DDJ readers.
        b_all_passed &= test("".into(), "?".into(), false);
        b_all_passed &= test("".into(), "*?".into(), false);
        b_all_passed &= test("".into(), "".into(), true);
        b_all_passed &= test("a".into(), "".into(), false);
    }

    if b_all_passed
    {
        println!("Passed wildcard tests");
    }
    else
    {
        println!("Failed wildcard tests");
    }
}


// A set of tests with no '*' wildcards.
//
fn test_tame()
{
    let mut i_reps: i32;
    let mut b_all_passed: bool = true;

	if COMPARE_PERFORMANCE
	{
		// Can choose as many repetitions as you might expect in production.
		i_reps = 1000000;
	}
	else
	{
		i_reps = 1;
	}

    while i_reps > 0
	{
        i_reps -= 1;

		// Case with last character mismatch.
        b_all_passed &= test("abc".into(), "abd".into(), false);

        // Cases with repeating character sequences.
        b_all_passed &= test("abcccd".into(), "abcccd".into(), true);
        b_all_passed &= test("mississipissippi".into(), 
		    "mississipissippi".into(), true);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), 
		    "xxxxzzzzzzzzyfffff".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "xxxxzzzzzzzzyf".into(), 
		    true);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "xxxxzzy.fffff".into(), 
		    false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "xxxxzzzzzzzzyf".into(), 
		    true);
        b_all_passed &= test("xyxyxyzyxyz".into(), "xyxyxyzyxyz".into(), true);
        b_all_passed &= test("mississippi".into(), "mississippi".into(), true);
        b_all_passed &= test("xyxyxyxyz".into(), "xyxyxyxyz".into(), true);
        b_all_passed &= test("m ississippi".into(), "m ississippi".into(), 
		    true);
        b_all_passed &= test("ababac".into(), "ababac?".into(), false);
        b_all_passed &= test("dababac".into(), "ababac".into(), false);
        b_all_passed &= test("aaazz".into(), "aaazz".into(), true);
        b_all_passed &= test("a12b12".into(), "1212".into(), false);
        b_all_passed &= test("a12b12".into(), "a12b".into(), false);
        b_all_passed &= test("a12b12".into(), "a12b12".into(), true);

        // A mix of cases
        b_all_passed &= test("n".into(), "n".into(), true);
        b_all_passed &= test("aabab".into(), "aabab".into(), true);
        b_all_passed &= test("ar".into(), "ar".into(), true);
        b_all_passed &= test("aar".into(), "aaar".into(), false);
        b_all_passed &= test("XYXYXYZYXYz".into(), "XYXYXYZYXYz".into(), true);
        b_all_passed &= test("missisSIPpi".into(), "missisSIPpi".into(), true);
        b_all_passed &= test("mississipPI".into(), "mississipPI".into(), true);
        b_all_passed &= test("xyxyxyxyz".into(), "xyxyxyxyz".into(), true);
        b_all_passed &= test("miSsissippi".into(), "miSsissippi".into(), 
		    true);
        b_all_passed &= test("miSsissippi".into(), "miSsisSippi".into(), 
		    false);
        b_all_passed &= test("abAbac".into(), "abAbac".into(), true);
        b_all_passed &= test("abAbac".into(), "abAbac".into(), true);
        b_all_passed &= test("aAazz".into(), "aAazz".into(), true);
        b_all_passed &= test("A12b12".into(), "A12b123".into(), false);
        b_all_passed &= test("a12B12".into(), "a12B12".into(), true);
        b_all_passed &= test("oWn".into(), "oWn".into(), true);
        b_all_passed &= test("bLah".into(), "bLah".into(), true);
        b_all_passed &= test("bLah".into(), "bLaH".into(), false);

        // Single '?' cases.
        b_all_passed &= test("a".into(), "a".into(), true);
        b_all_passed &= test("ab".into(), "a?".into(), true);
        b_all_passed &= test("abc".into(), "ab?".into(), true);

        // Mixed '?' cases.
        b_all_passed &= test("a".into(), "??".into(), false);
        b_all_passed &= test("ab".into(), "??".into(), true);
        b_all_passed &= test("abc".into(), "???".into(), true);
        b_all_passed &= test("abcd".into(), "????".into(), true);
        b_all_passed &= test("abc".into(), "????".into(), false);
        b_all_passed &= test("abcd".into(), "?b??".into(), true);
        b_all_passed &= test("abcd".into(), "?a??".into(), false);
        b_all_passed &= test("abcd".into(), "??c?".into(), true);
        b_all_passed &= test("abcd".into(), "??d?".into(), false);
        b_all_passed &= test("abcde".into(), "?b?d*?".into(), true);

        // Longer string scenarios.
        b_all_passed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".into(), 
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".into(), true);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), true);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajaxalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), false);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaggggagaaaaaaaab".into(), false);
        b_all_passed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab".into(), true);
        b_all_passed &= test("aaabbaabbaab".into(), "aaabbaabbaab".into(), 
		    true);
        b_all_passed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(), 
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(), true);
        b_all_passed &= test("aaaaaaaaaaaaaaaaa".into(), 
            "aaaaaaaaaaaaaaaaa".into(), true);
        b_all_passed &= test("aaaaaaaaaaaaaaaa".into(), 
            "aaaaaaaaaaaaaaaaa".into(), false);
        b_all_passed &= test("abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn".into(), 
            "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".into(), 
			false);
        b_all_passed &= test("abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn".into(), 
            "abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn".into(), 
			true);
        b_all_passed &= test("abcabcdabcdabcabcd".into(), 
		    "abcabc?abcabcabc".into(), false);
        b_all_passed &= test(
            "abcabcdabcdabcabcdabcdabcabcdabcabcabcd".into(), 
            "abcabc?abc?abcabc?abc?abc?bc?abc?bc?bcd".into(), true);
        b_all_passed &= test("?abc?".into(), "?abc?".into(), true);
    }

    if b_all_passed
    {
        println!("Passed tame string tests");
    }
    else
    {
        println!("Failed tame string tests");
    }
}


// A set of tests with empty input.
//
fn test_empty()
{
    let mut i_reps: i32;
    let mut b_all_passed: bool = true;

	if COMPARE_PERFORMANCE
	{
		// Can choose as many repetitions as you might expect in production.
		i_reps = 1000000;
	}
	else
	{
		i_reps = 1;
	}

    while i_reps > 0
	{
		i_reps -= 1;
	
		// A simple case.
        b_all_passed &= test("".into(), "abd".into(), false);

        // Cases with repeating character sequences.
        b_all_passed &= test("".into(), "abcccd".into(), false);
        b_all_passed &= test("".into(), "mississipissippi".into(), false);
        b_all_passed &= test("".into(), "xxxxzzzzzzzzyfffff".into(), false);
        b_all_passed &= test("".into(), "xxxxzzzzzzzzyf".into(), false);
        b_all_passed &= test("".into(), "xxxxzzy.fffff".into(), false);
        b_all_passed &= test("".into(), "xxxxzzzzzzzzyf".into(), false);
        b_all_passed &= test("".into(), "xyxyxyzyxyz".into(), false);
        b_all_passed &= test("".into(), "mississippi".into(), false);
        b_all_passed &= test("".into(), "xyxyxyxyz".into(), false);
        b_all_passed &= test("".into(), "m ississippi".into(), false);
        b_all_passed &= test("".into(), "ababac*".into(), false);
        b_all_passed &= test("".into(), "ababac".into(), false);
        b_all_passed &= test("".into(), "aaazz".into(), false);
        b_all_passed &= test("".into(), "1212".into(), false);
        b_all_passed &= test("".into(), "a12b".into(), false);
        b_all_passed &= test("".into(), "a12b12".into(), false);

        // A mix of cases.
        b_all_passed &= test("".into(), "n".into(), false);
        b_all_passed &= test("".into(), "aabab".into(), false);
        b_all_passed &= test("".into(), "ar".into(), false);
        b_all_passed &= test("".into(), "aaar".into(), false);
        b_all_passed &= test("".into(), "XYXYXYZYXYz".into(), false);
        b_all_passed &= test("".into(), "missisSIPpi".into(), false);
        b_all_passed &= test("".into(), "mississipPI".into(), false);
        b_all_passed &= test("".into(), "xyxyxyxyz".into(), false);
        b_all_passed &= test("".into(), "miSsissippi".into(), false);
        b_all_passed &= test("".into(), "miSsisSippi".into(), false);
        b_all_passed &= test("".into(), "abAbac".into(), false);
        b_all_passed &= test("".into(), "abAbac".into(), false);
        b_all_passed &= test("".into(), "aAazz".into(), false);
        b_all_passed &= test("".into(), "A12b123".into(), false);
        b_all_passed &= test("".into(), "a12B12".into(), false);
        b_all_passed &= test("".into(), "oWn".into(), false);
        b_all_passed &= test("".into(), "bLah".into(), false);
        b_all_passed &= test("".into(), "bLaH".into(), false);

		// Both strings empty.
        b_all_passed &= test("".into(), "".into(), true);

		// Another simple case.
        b_all_passed &= test("abc".into(), "".into(), false);

        // More cases with repeating character sequences.
        b_all_passed &= test("abcccd".into(), "".into(), false);
        b_all_passed &= test("mississipissippi".into(), "".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "".into(), false);
        b_all_passed &= test("xxxxzzzzzzzzyf".into(), "".into(), false);
        b_all_passed &= test("xyxyxyzyxyz".into(), "".into(), false);
        b_all_passed &= test("mississippi".into(), "".into(), false);
        b_all_passed &= test("xyxyxyxyz".into(), "".into(), false);
        b_all_passed &= test("m ississippi".into(), "".into(), false);
        b_all_passed &= test("ababac".into(), "".into(), false);
        b_all_passed &= test("dababac".into(), "".into(), false);
        b_all_passed &= test("aaazz".into(), "".into(), false);
        b_all_passed &= test("a12b12".into(), "".into(), false);
        b_all_passed &= test("a12b12".into(), "".into(), false);
        b_all_passed &= test("a12b12".into(), "".into(), false);

        // Another mix of cases.
        b_all_passed &= test("n".into(), "".into(), false);
        b_all_passed &= test("aabab".into(), "".into(), false);
        b_all_passed &= test("ar".into(), "".into(), false);
        b_all_passed &= test("aar".into(), "".into(), false);
        b_all_passed &= test("XYXYXYZYXYz".into(), "".into(), false);
        b_all_passed &= test("missisSIPpi".into(), "".into(), false);
        b_all_passed &= test("mississipPI".into(), "".into(), false);
        b_all_passed &= test("xyxyxyxyz".into(), "".into(), false);
        b_all_passed &= test("miSsissippi".into(), "".into(), false);
        b_all_passed &= test("miSsissippi".into(), "".into(), false);
        b_all_passed &= test("abAbac".into(), "".into(), false);
        b_all_passed &= test("abAbac".into(), "".into(), false);
        b_all_passed &= test("aAazz".into(), "".into(), false);
        b_all_passed &= test("A12b12".into(), "".into(), false);
        b_all_passed &= test("a12B12".into(), "".into(), false);
        b_all_passed &= test("oWn".into(), "".into(), false);
        b_all_passed &= test("bLah".into(), "".into(), false);
        b_all_passed &= test("bLah".into(), "".into(), false);
    }

    if b_all_passed
    {
        println!("Passed empty string tests");
    }
    else
    {
        println!("Failed empty string tests");
    }
}


// Correctness tests for a case-insensitive arrangement for invoking a 
// UTF-8-enabled routine for matching wildcards.  See relevant code / 
// comments in test().
//
fn test_utf8()
{
    let mut b_all_passed: bool = true;

	// Simple correctness tests involving various UTF-8 symbols and 
	// international content.
    b_all_passed &= test("🐂🚀♥🍀貔貅🦁★□√🚦€¥☯🐴😊🍓🐕🎺🧊☀☂🐉".into(), 
	                     "*☂🐉".into(), true);
    b_all_passed &= test("AbCD".into(), "abc?".into(), true);
    b_all_passed &= test("AbC★".into(), "abc?".into(), true);
	b_all_passed &= test("▲●🐎✗🤣🐶♫🌻ॐ".into(), "▲●☂*".into(), false);
	b_all_passed &= test("𓋍𓋔𓎍".into(), "𓋍𓋔?".into(), true);
	b_all_passed &= test("𓋍𓋔𓎍".into(), "𓋍?𓋔𓎍".into(), false);
	b_all_passed &= test("♅☌♇".into(), "♅☌♇".into(), true);
	b_all_passed &= test("⚛⚖☁".into(), "⚛🍄☁".into(), false);
	b_all_passed &= test("⚛⚖☁o".into(), "⚛⚖☁O".into(), true);
	b_all_passed &= test("⚛⚖☁O".into(), "⚛⚖☁0".into(), false);
	b_all_passed &= test("गते गते पारगते पारसंगते बोधि स्वाहा".into(), 
	                     "गते गते पारगते प????गते बोधि स्वाहा".into(), true);
	b_all_passed &= test(
	    "Мне нужно выучить русский язык, чтобы лучше оценить Пушкина.".into(), 
	    "Мне нужно выучить * язык, чтобы лучше оценить *.".into(), true);
	b_all_passed &= test(
	    "אני צריך ללמוד אנגלית כדי להעריך את גינסברג".into(), 
	    " אני צריך ללמוד אנגלית כדי להעריך את ???????".into(), false);
	b_all_passed &= test(
	    "ગિન્સબર્ગની શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે અંગ્રેજી શીખવું પડશે.".into(), 
	    "* શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે * શીખવું પડશે.".into(), true);
	b_all_passed &= test(
	    "ગિન્સબર્ગની શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે અંગ્રેજી શીખવું પડશે.".into(), 
	    "??????????? શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે * શીખવું પડશે.".into(), true);
	b_all_passed &= test(
	    "ગિન્સબર્ગની શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે અંગ્રેજી શીખવું પડશે.".into(), 
	    "ગિન્સબર્ગની શ્રેષ્ઠ પ્રશંસા કરવા માટે મારે હિબ્રુ ભાષા શીખવી પડશે.".into(), false);
	
	// These tests involve multiple=byte code points that contain bytes 
	// identical to the single-byte code points for '*' and '?'.
	b_all_passed &= test("ḪؿꜪἪꜿ".into(), "ḪؿꜪἪꜿ".into(), true);
	b_all_passed &= test("ḪؿUἪꜿ".into(), "ḪؿꜪἪꜿ".into(), false);
	b_all_passed &= test("ḪؿꜪἪꜿ".into(), "ḪؿꜪἪꜿЖ".into(), false);
	b_all_passed &= test("ḪؿꜪἪꜿ".into(), "ЬḪؿꜪἪꜿ".into(), false);
	b_all_passed &= test("ḪؿꜪἪꜿ".into(), "?ؿꜪ*ꜿ".into(), true);

	if b_all_passed
    {
        println!("Passed UTF-8 tests");
    }
    else
    {
        println!("Failed UTF-8 tests");
    }
}


// Entry point for the Rust executable.  Performance findings (if any) are 
// displayed here, once all tests have run.
//
fn main()
{
	// Accumulate timing data for 4 versions of the algorithm.
	if COMPARE_TAME
	{
		test_tame();
	}

	if COMPARE_EMPTY
	{
		test_empty();
	}

	if COMPARE_WILD
	{
		test_wild();
	}
	
	if TEST_UTF8
	{
		test_utf8();
	}

	if COMPARE_PERFORMANCE
	{
		unsafe  // Timings have been accumulated via mutable file-scope data.
		{
			let base: f64 = 10.0;
			let f_cumulative_time_ascii_version: f64 = 
			      (U_RUST_TIME_ASCII as f64 / base.powf(9.0)).round() * 
				      base.powf(3.0);
			let f_cumulative_time_utf8_version: f64 = 
			      (U_RUST_TIME_UTF8 as f64 / base.powf(9.0)).round() * 
				       base.powf(3.0);
			let f_cumulative_time_fwc_cpp: f64 = 
			      (U_CPP_TIME_FASTEST as f64 / base.powf(9.0)).round() * 
				       base.powf(3.0);		 
			let f_cumulative_time_fwcp_cpp: f64 = 
			      (U_CPP_TIME_PORTABLE as f64 / base.powf(9.0)).round() * 
				       base.powf(3.0);

			// Represent the rounded timings in seconds, using integer values.
			let u_utf8_version_seconds = 
			    (f_cumulative_time_utf8_version as u64) / 1000;
			let u_ascii_version_seconds = 
			    (f_cumulative_time_ascii_version as u64) / 1000;
			let u_fwcp_cpp_seconds = 
			    (f_cumulative_time_fwcp_cpp as u64) / 1000;
			let u_fwc_cpp_seconds = 
			    (f_cumulative_time_fwc_cpp as u64) / 1000;

			// Show the timing results.
			println!(
				"fast_wild_compare_utf8 - \
				Rust version providing UTF-8 enablement: {:?} seconds", 
				u_utf8_version_seconds);
			println!(
				"fast_wild_compare_ascii - \
				Light-weight Rust version for string slices: {:?} seconds", 
				u_ascii_version_seconds);
			println!(
				"FastWildComparePortable - \
				C++ equivalent of fast_wild_compare_ascii: {:?} seconds", 
				u_fwcp_cpp_seconds);
			println!("FastWildCompare - \
			Optimized C++ pointer-based algorithm: {:?} seconds", 
				u_fwc_cpp_seconds);
		}
	}	
}

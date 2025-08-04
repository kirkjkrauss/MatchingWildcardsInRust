// FastWildCompare(), and related code
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
// This file provides C/C++ routines for matching wildcards and includes 
// a set of testcases for correctness and performance.
//
// The code is included with a Rust implementation, which is based on a 
// virtually identical algorithm with identical testcases, for performance 
// comparison between the implementations. 
//
#include <stdio.h>

//#define BUILD_A_CPP_EXE      1
//#define COMPARE_PERFORMANCE  1
#define COMPARE_WILD         1
#define COMPARE_TAME         1
#define COMPARE_EMPTY        1

// Compares two text strings.  Accepts '?' as a single-character wildcard.  
// For each '*' wildcard, seeks out a matching sequence of any characters 
// beyond it.  Otherwise compares the strings a character at a time. 
//
extern "C" bool FastWildCompare(char *pWild, char *pTame)
{
	char *pWildSequence;  // Points to prospective wild string match after '*'
	char *pTameSequence;  // Points to prospective tame string match

	// Find a first wildcard, if one exists, and the beginning of any  
	// prospectively matching sequence after it.
	do
	{
		// Check for the end from the start.  Get out fast, if possible.
		if (!*pTame)
		{
			if (*pWild)
			{
				while (*(pWild++) == '*')
				{
					if (!(*pWild))
					{
						return true;   // "ab" matches "ab*".
					}
				}

			    return false;          // "abcd" doesn't match "abc".
			}
			else
			{
				return true;           // "abc" matches "abc".
			}
		}
		else if (*pWild == '*')
		{
			// Got wild: set up for the second loop and skip on down there.
			while (*(++pWild) == '*')
			{
				continue;
			}

			if (!*pWild)
			{
				return true;           // "abc*" matches "abcd".
			}

			// Search for the next prospective match.
			if (*pWild != '?')
			{
				while (*pWild != *pTame)
				{
					if (!*(++pTame))
					{
						return false;  // "a*bc" doesn't match "ab".
					}
				}
			}

			// Keep fallback positions for retry in case of incomplete match.
			pWildSequence = pWild;
			pTameSequence = pTame;
			break;
		}
		else if (*pWild != *pTame && *pWild != '?')
		{
			return false;              // "abc" doesn't match "abd".
		}

		++pWild;                       // Everything's a match, so far.
		++pTame;
	} while (true);

	// Find any further wildcards and any further matching sequences.
	do
	{
		if (*pWild == '*')
		{
			// Got wild again.
			while (*(++pWild) == '*')
			{
				continue;
			}

			if (!*pWild)
			{
				return true;           // "ab*c*" matches "abcd".
			}

			if (!*pTame)
			{
				return false;          // "*bcd*" doesn't match "abc".
			}

			// Search for the next prospective match.
			if (*pWild != '?')
			{
				while (*pWild != *pTame)
				{
					if (!*(++pTame))
					{
						return false;  // "a*b*c" doesn't match "ab".
					}
				}
			}

			// Keep the new fallback positions.
			pWildSequence = pWild;
			pTameSequence = pTame;
		}
		else if (*pWild != *pTame && *pWild != '?')
		{
			// The equivalent portion of the upper loop is really simple.
			if (!*pTame)
			{
				return false;          // "*bcd" doesn't match "abc".
			}

			// A fine time for questions.
			while (*pWildSequence == '?')
			{
				++pWildSequence;
				++pTameSequence;
			}

			pWild = pWildSequence;

			// Fall back, but never so far again.
			while (*pWild != *(++pTameSequence))
			{
				if (!*pTameSequence)
				{
					return false;      // "*a*b" doesn't match "ac".
				}
			}

			pTame = pTameSequence;
		}

		// Another check for the end, at the end.
		if (!*pTame)
		{
			if (!*pWild)
			{
				return true;           // "*bc" matches "abc".
			}
			else
			{
				return false;          // "*bc" doesn't match "abcd".
			}
		}

		++pWild;                       // Everything's still a match.
		++pTame;
	} while (true);
}


// Slower but portable version of FastWildCompare().  Performs no direct 
// pointer manipulation.  Can work with wide-character text strings.  Use 
// only with null-terminated strings.
//
// Compares two text strings.  Accepts '?' as a single-character wildcard.  
// For each '*' wildcard, seeks out a matching sequence of any characters 
// beyond it.  Otherwise compares the strings a character at a time.
//
extern "C" bool FastWildComparePortable(char *strWild, char *strTame)
{
	int  iWild = 0;     // Index for both tame and wild strings in upper loop
	int  iTame;         // Index for tame string, set going into lower loop
	int  iWildSequence; // Index for prospective match after '*' (wild string)
	int  iTameSequence; // Index for prospective match (tame string)

	// Find a first wildcard, if one exists, and the beginning of any  
	// prospectively matching sequence after it.
	do
	{
		// Check for the end from the start.  Get out fast, if possible.
		if (!strTame[iWild])
		{
			if (strWild[iWild])
			{
				while (strWild[iWild++] == '*')
				{
					if (!strWild[iWild])
					{
						return true;   // "ab" matches "ab*".
					}
				}

			    return false;          // "abcd" doesn't match "abc".
			}
			else
			{
				return true;           // "abc" matches "abc".
			}
		}
		else if (strWild[iWild] == '*')
		{
			// Got wild: set up for the second loop and skip on down there.
			iTame = iWild;

			while (strWild[++iWild] == '*')
			{
				continue;
			}

			if (!strWild[iWild])
			{
				return true;           // "abc*" matches "abcd".
			}

			// Search for the next prospective match.
			if (strWild[iWild] != '?')
			{
				while (strWild[iWild] != strTame[iTame])
				{
					if (!strTame[++iTame])
					{
						return false;  // "a*bc" doesn't match "ab".
					}
				}
			}

			// Keep fallback positions for retry in case of incomplete match.
			iWildSequence = iWild;
			iTameSequence = iTame;
			break;
		}
		else if (strWild[iWild] != strTame[iWild] && strWild[iWild] != '?')
		{
			return false;              // "abc" doesn't match "abd".
		}

		++iWild;                       // Everything's a match, so far.
	} while (true);

	// Find any further wildcards and any further matching sequences.
	do
	{
		if (strWild[iWild] == '*')
		{
			// Got wild again.
			while (strWild[++iWild] == '*')
			{
				continue;
			}

			if (!strWild[iWild])
			{
				return true;           // "ab*c*" matches "abcd".
			}

			if (!strTame[iTame])
			{
				return false;          // "*bcd*" doesn't match "abc".
			}

			// Search for the next prospective match.
			if (strWild[iWild] != '?')
			{
				while (strWild[iWild] != strTame[iTame])
				{
					if (!strTame[++iTame])
					{
						return false;  // "a*b*c" doesn't match "ab".
					}
				}
			}

			// Keep the new fallback positions.
			iWildSequence = iWild;
			iTameSequence = iTame;
		}
		else if (strWild[iWild] != strTame[iTame] && strWild[iWild] != '?')
		{
			// The equivalent portion of the upper loop is really simple.
			if (!strTame[iTame])
			{
				return false;          // "*bcd" doesn't match "abc".
			}

			// A fine time for questions.
			while (strWild[iWildSequence] == '?')
			{
				++iWildSequence;
				++iTameSequence;
			}

			iWild = iWildSequence;

			// Fall back, but never so far again.
			while (strWild[iWild] != strTame[++iTameSequence])
			{
				if (!strTame[iTameSequence])
				{
					return false;      // "*a*b" doesn't match "ac".
				}
			}

			iTame = iTameSequence;
		}

		// Another check for the end, at the end.
		if (!strTame[iTame])
		{
			if (!strWild[iWild])
			{
				return true;           // "*bc" matches "abc".
			}
			else
			{
				return false;          // "*bc" doesn't match "abcd".
			}
		}

		++iWild;                       // Everything's still a match.
		++iTame;
	} while (true);
}


// This function compares a tame/wild string pair via each included routine.
//
bool test(char *pTame, char *pWild, bool bExpectedResult)
{
	bool bPassed = true;

	if (bExpectedResult != FastWildCompare(pWild, pTame))
	{
		bPassed = false;
	}

	if (bExpectedResult != FastWildComparePortable(pWild, pTame))
	{
		bPassed = false;
	}

	return bPassed;
}


// A set of wildcard comparison tests.
//
int testwild(void)
{
    int  nReps;
    bool bAllPassed = true;

#if defined(COMPARE_PERFORMANCE)
    // Can choose as many repetitions as you're expecting in the real world.
    nReps = 1000000;
#else
    nReps = 1;
#endif

    while (nReps--)
    {
		// Case with first wildcard after total match.
        bAllPassed &= test("Hi", "Hi*", true);
		
		// Case with mismatch after '*'
        bAllPassed &= test("abc", "ab*d", false);

        // Cases with repeating character sequences.
        bAllPassed &= test("abcccd", "*ccd", true);
        bAllPassed &= test("mississipissippi", "*issip*ss*", true);
        bAllPassed &= test("xxxx*zzzzzzzzy*f", "xxxx*zzy*fffff", false);
        bAllPassed &= test("xxxx*zzzzzzzzy*f", "xxx*zzy*f", true);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxx*zzy*fffff", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxx*zzy*f", true);
        bAllPassed &= test("xyxyxyzyxyz", "xy*z*xyz", true);
        bAllPassed &= test("mississippi", "*sip*", true);
        bAllPassed &= test("xyxyxyxyz", "xy*xyz", true);
        bAllPassed &= test("mississippi", "mi*sip*", true);
        bAllPassed &= test("ababac", "*abac*", true);
        bAllPassed &= test("ababac", "*abac*", true);
        bAllPassed &= test("aaazz", "a*zz*", true);
        bAllPassed &= test("a12b12", "*12*23", false);
        bAllPassed &= test("a12b12", "a12b", false);
        bAllPassed &= test("a12b12", "*12*12*", true);

#if !defined(COMPARE_PERFORMANCE)
		// From DDJ reader Andy Belf: a case of repeating text matching the 
		// different kinds of wildcards in order of '*' and then '?'.
        bAllPassed &= test("caaab", "*a?b", true);
		// This similar case was found, probably independently, by Dogan Kurt.
        bAllPassed &= test("aaaaa", "*aa?", true);
#endif

        // Additional cases where the '*' char appears in the tame string.
        bAllPassed &= test("*", "*", true);
        bAllPassed &= test("a*abab", "a*b", true);
        bAllPassed &= test("a*r", "a*", true);
        bAllPassed &= test("a*ar", "a*aar", false);

        // More double wildcard scenarios.
        bAllPassed &= test("XYXYXYZYXYz", "XY*Z*XYz", true);
        bAllPassed &= test("missisSIPpi", "*SIP*", true);
        bAllPassed &= test("mississipPI", "*issip*PI", true);
        bAllPassed &= test("xyxyxyxyz", "xy*xyz", true);
        bAllPassed &= test("miSsissippi", "mi*sip*", true);
        bAllPassed &= test("miSsissippi", "mi*Sip*", false);
        bAllPassed &= test("abAbac", "*Abac*", true);
        bAllPassed &= test("abAbac", "*Abac*", true);
        bAllPassed &= test("aAazz", "a*zz*", true);
        bAllPassed &= test("A12b12", "*12*23", false);
        bAllPassed &= test("a12B12", "*12*12*", true);
        bAllPassed &= test("oWn", "*oWn*", true);

        // Completely tame (no wildcards) cases.
        bAllPassed &= test("bLah", "bLah", true);
        bAllPassed &= test("bLah", "bLaH", false);

        // Simple mixed wildcard tests suggested by Marlin Deckert.
        bAllPassed &= test("a", "*?", true);
        bAllPassed &= test("ab", "*?", true);
        bAllPassed &= test("abc", "*?", true);

        // More mixed wildcard tests including coverage for false positives.
        bAllPassed &= test("a", "??", false);
        bAllPassed &= test("ab", "?*?", true);
        bAllPassed &= test("ab", "*?*?*", true);
        bAllPassed &= test("abc", "?**?*?", true);
        bAllPassed &= test("abc", "?**?*&?", false);
        bAllPassed &= test("abcd", "?b*??", true);
        bAllPassed &= test("abcd", "?a*??", false);
        bAllPassed &= test("abcd", "?**?c?", true);
        bAllPassed &= test("abcd", "?**?d?", false);
        bAllPassed &= test("abcde", "?*b*?*d*?", true);

        // Single-character-match cases.
        bAllPassed &= test("bLah", "bL?h", true);
        bAllPassed &= test("bLaaa", "bLa?", false);
        bAllPassed &= test("bLah", "bLa?", true);
        bAllPassed &= test("bLaH", "?Lah", false);
        bAllPassed &= test("bLaH", "?LaH", true);

        // Many-wildcard scenarios.
        bAllPassed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", 
            "a*a*a*a*a*a*aa*aaa*a*a*b", true);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "*a*b*ba*ca*a*aa*aaa*fa*ga*b*", true);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "*a*b*ba*ca*a*x*aaa*fa*ga*b*", false);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "*a*b*ba*ca*aaaa*fa*ga*gggg*b*", false);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "*a*b*ba*ca*aaaa*fa*ga*ggg*b*", true);
        bAllPassed &= test("aaabbaabbaab", "*aabbaa*a*", true);
        bAllPassed &= test("a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", 
            "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", true);
        bAllPassed &= test("aaaaaaaaaaaaaaaaa", 
            "*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", true);
        bAllPassed &= test("aaaaaaaaaaaaaaaa", 
            "*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*", false);
        bAllPassed &= test("abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*a\
bcdefghij*abcdefghijk*abcdefghijkl*abcdefghijklm*abcdefghijklmn", 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*a\
            bc*", false);
        bAllPassed &= test("abc*abcd*abcde*abcdef*abcdefg*abcdefgh*abcdefghi*a\
bcdefghij*abcdefghijk*abcdefghijkl*abcdefghijklm*abcdefghijklmn", 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*", true);
        bAllPassed &= test("abc*abcd*abcd*abc*abcd", "abc*abc*abc*abc*abc", 
            false);
        bAllPassed &= test(
            "abc*abcd*abcd*abc*abcd*abcd*abc*abcd*abc*abc*abcd", 
            "abc*abc*abc*abc*abc*abc*abc*abc*abc*abc*abcd", true);
        bAllPassed &= test("abc", "********a********b********c********", 
            true);
        bAllPassed &= test("********a********b********c********", "abc", 
            false);
        bAllPassed &= test("abc", "********a********b********b********", 
            false);
        bAllPassed &= test("*abc*", "***a*b*c***", true);

        // A case-insensitive algorithm test.
        // bAllPassed &= test("mississippi", "*issip*PI", true);

        // Tests suggested by other DDJ readers
        bAllPassed &= test("", "?", false);
        bAllPassed &= test("", "*?", false);
        bAllPassed &= test("", "", true);
        bAllPassed &= test("a", "", false);
    }

    if (bAllPassed)
    {
        printf("Passed\n");
    }
    else
    {
        printf("Failed\n");
    }

    return 0;
}


// A set of tests with no '*' wildcards.
//
int testtame(void)
{
    int  nReps;
    bool bAllPassed = true;

#if defined(COMPARE_PERFORMANCE)
    // Can choose as many repetitions as you're expecting in the real world.
    nReps = 1000000;
#else
    nReps = 1;
#endif

    while (nReps--)
    {
		// Case with last character mismatch.
        bAllPassed &= test("abc", "abd", false);

        // Cases with repeating character sequences.
        bAllPassed &= test("abcccd", "abcccd", true);
        bAllPassed &= test("mississipissippi", "mississipissippi", true);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxxzzzzzzzzyfffff", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxxzzzzzzzzyf", true);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxxzzy.fffff", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "xxxxzzzzzzzzyf", true);
        bAllPassed &= test("xyxyxyzyxyz", "xyxyxyzyxyz", true);
        bAllPassed &= test("mississippi", "mississippi", true);
        bAllPassed &= test("xyxyxyxyz", "xyxyxyxyz", true);
        bAllPassed &= test("m ississippi", "m ississippi", true);
        bAllPassed &= test("ababac", "ababac?", false);
        bAllPassed &= test("dababac", "ababac", false);
        bAllPassed &= test("aaazz", "aaazz", true);
        bAllPassed &= test("a12b12", "1212", false);
        bAllPassed &= test("a12b12", "a12b", false);
        bAllPassed &= test("a12b12", "a12b12", true);

        // A mix of cases
        bAllPassed &= test("n", "n", true);
        bAllPassed &= test("aabab", "aabab", true);
        bAllPassed &= test("ar", "ar", true);
        bAllPassed &= test("aar", "aaar", false);
        bAllPassed &= test("XYXYXYZYXYz", "XYXYXYZYXYz", true);
        bAllPassed &= test("missisSIPpi", "missisSIPpi", true);
        bAllPassed &= test("mississipPI", "mississipPI", true);
        bAllPassed &= test("xyxyxyxyz", "xyxyxyxyz", true);
        bAllPassed &= test("miSsissippi", "miSsissippi", true);
        bAllPassed &= test("miSsissippi", "miSsisSippi", false);
        bAllPassed &= test("abAbac", "abAbac", true);
        bAllPassed &= test("abAbac", "abAbac", true);
        bAllPassed &= test("aAazz", "aAazz", true);
        bAllPassed &= test("A12b12", "A12b123", false);
        bAllPassed &= test("a12B12", "a12B12", true);
        bAllPassed &= test("oWn", "oWn", true);
        bAllPassed &= test("bLah", "bLah", true);
        bAllPassed &= test("bLah", "bLaH", false);

        // Single '?' cases.
        bAllPassed &= test("a", "a", true);
        bAllPassed &= test("ab", "a?", true);
        bAllPassed &= test("abc", "ab?", true);

        // Mixed '?' cases.
        bAllPassed &= test("a", "??", false);
        bAllPassed &= test("ab", "??", true);
        bAllPassed &= test("abc", "???", true);
        bAllPassed &= test("abcd", "????", true);
        bAllPassed &= test("abc", "????", false);
        bAllPassed &= test("abcd", "?b??", true);
        bAllPassed &= test("abcd", "?a??", false);
        bAllPassed &= test("abcd", "??c?", true);
        bAllPassed &= test("abcd", "??d?", false);
        bAllPassed &= test("abcde", "?b?d*?", true);

        // Longer string scenarios.
        bAllPassed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", 
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", true);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", true);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajaxalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", false);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaggggagaaaaaaaab", false);
        bAllPassed &= test("abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", 
            "abababababababababababababababababababaacacacacaca\
cacadaeafagahaiajakalaaaaaaaaaaaaaaaaaffafagaagggagaaaaaaaab", true);
        bAllPassed &= test("aaabbaabbaab", "aaabbaabbaab", true);
        bAllPassed &= test("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true);
        bAllPassed &= test("aaaaaaaaaaaaaaaaa", 
            "aaaaaaaaaaaaaaaaa", true);
        bAllPassed &= test("aaaaaaaaaaaaaaaa", 
            "aaaaaaaaaaaaaaaaa", false);
        bAllPassed &= test("abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn", 
            "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc", 
			false);
        bAllPassed &= test("abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn", 
            "abcabcdabcdeabcdefabcdefgabcdefghabcdefghia\
bcdefghijabcdefghijkabcdefghijklabcdefghijklmabcdefghijklmn", 
			true);
        bAllPassed &= test("abcabcdabcdabcabcd", "abcabc?abcabcabc", 
            false);
        bAllPassed &= test(
            "abcabcdabcdabcabcdabcdabcabcdabcabcabcd", 
            "abcabc?abc?abcabc?abc?abc?bc?abc?bc?bcd", true);
        bAllPassed &= test("?abc?", "?abc?", true);
    }

    if (bAllPassed)
    {
        printf("Passed\n");
    }
    else
    {
        printf("Failed\n");
    }

    return 0;
}


// A set of tests with empty strings.
//
int testempty(void)
{
    int  nReps;
    bool bAllPassed = true;

#if defined(COMPARE_PERFORMANCE)
    // Can choose as many repetitions as you're expecting in the real world.
    nReps = 1000000;
#else
    nReps = 1;
#endif

    while (nReps--)
    {
		// A simple case
        bAllPassed &= test("", "abd", false);

        // Cases with repeating character sequences
        bAllPassed &= test("", "abcccd", false);
        bAllPassed &= test("", "mississipissippi", false);
        bAllPassed &= test("", "xxxxzzzzzzzzyfffff", false);
        bAllPassed &= test("", "xxxxzzzzzzzzyf", false);
        bAllPassed &= test("", "xxxxzzy.fffff", false);
        bAllPassed &= test("", "xxxxzzzzzzzzyf", false);
        bAllPassed &= test("", "xyxyxyzyxyz", false);
        bAllPassed &= test("", "mississippi", false);
        bAllPassed &= test("", "xyxyxyxyz", false);
        bAllPassed &= test("", "m ississippi", false);
        bAllPassed &= test("", "ababac*", false);
        bAllPassed &= test("", "ababac", false);
        bAllPassed &= test("", "aaazz", false);
        bAllPassed &= test("", "1212", false);
        bAllPassed &= test("", "a12b", false);
        bAllPassed &= test("", "a12b12", false);

        // A mix of cases
        bAllPassed &= test("", "n", false);
        bAllPassed &= test("", "aabab", false);
        bAllPassed &= test("", "ar", false);
        bAllPassed &= test("", "aaar", false);
        bAllPassed &= test("", "XYXYXYZYXYz", false);
        bAllPassed &= test("", "missisSIPpi", false);
        bAllPassed &= test("", "mississipPI", false);
        bAllPassed &= test("", "xyxyxyxyz", false);
        bAllPassed &= test("", "miSsissippi", false);
        bAllPassed &= test("", "miSsisSippi", false);
        bAllPassed &= test("", "abAbac", false);
        bAllPassed &= test("", "abAbac", false);
        bAllPassed &= test("", "aAazz", false);
        bAllPassed &= test("", "A12b123", false);
        bAllPassed &= test("", "a12B12", false);
        bAllPassed &= test("", "oWn", false);
        bAllPassed &= test("", "bLah", false);
        bAllPassed &= test("", "bLaH", false);

		// Both strings empty
        bAllPassed &= test("", "", true);

		// Another simple case
        bAllPassed &= test("abc", "", false);

        // Cases with repeating character sequences.
        bAllPassed &= test("abcccd", "", false);
        bAllPassed &= test("mississipissippi", "", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "", false);
        bAllPassed &= test("xxxxzzzzzzzzyf", "", false);
        bAllPassed &= test("xyxyxyzyxyz", "", false);
        bAllPassed &= test("mississippi", "", false);
        bAllPassed &= test("xyxyxyxyz", "", false);
        bAllPassed &= test("m ississippi", "", false);
        bAllPassed &= test("ababac", "", false);
        bAllPassed &= test("dababac", "", false);
        bAllPassed &= test("aaazz", "", false);
        bAllPassed &= test("a12b12", "", false);
        bAllPassed &= test("a12b12", "", false);
        bAllPassed &= test("a12b12", "", false);

        // A mix of cases
        bAllPassed &= test("n", "", false);
        bAllPassed &= test("aabab", "", false);
        bAllPassed &= test("ar", "", false);
        bAllPassed &= test("aar", "", false);
        bAllPassed &= test("XYXYXYZYXYz", "", false);
        bAllPassed &= test("missisSIPpi", "", false);
        bAllPassed &= test("mississipPI", "", false);
        bAllPassed &= test("xyxyxyxyz", "", false);
        bAllPassed &= test("miSsissippi", "", false);
        bAllPassed &= test("miSsissippi", "", false);
        bAllPassed &= test("abAbac", "", false);
        bAllPassed &= test("abAbac", "", false);
        bAllPassed &= test("aAazz", "", false);
        bAllPassed &= test("A12b12", "", false);
        bAllPassed &= test("a12B12", "", false);
        bAllPassed &= test("oWn", "", false);
        bAllPassed &= test("bLah", "", false);
        bAllPassed &= test("bLah", "", false);
    }

    if (bAllPassed)
    {
        printf("Passed\n");
    }
    else
    {
        printf("Failed\n");
    }

    return 0;
}


// Entry point for an executable that may be built to invoke the above 
// routines.
//
#if defined(BUILD_A_CPP_EXE)
int main(void)
{
#if defined(COMPARE_TAME)
	testtame();
#endif

#if defined(COMPARE_EMPTY)
	testempty();
#endif

#if defined(COMPARE_WILD)
	testwild();
#endif

	return 0;
}
#endif  // defined(BUILD_A_CPP_EXE)
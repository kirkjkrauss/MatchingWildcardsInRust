// Rust routines for matching wildcards.
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

// Rust implementation of FastWildCompare(), for ASCII text.
//
// Compares two ASCII &str's.  Accepts '?' as a single-character wildcard.
// For each '*' wildcard, seeks out a matching sequence of any characters 
// beyond it.  Otherwise compares the &str's a character at a time. 
//
pub fn fast_wild_compare_ascii(
          wild_str: &str, 
          tame_str: &str) -> bool
{
	let mut iwild: usize = 0;  // Index for both input &str's in upper loop
	let mut itame: usize;      // Index for tame &str, used in lower loop
	let mut iwild_sequence: usize; // Index for prospective match after '*'
	let mut itame_sequence: usize; // Index for prospective match in tame &str

    // Find a first wildcard, if one exists, and the beginning of any  
    // prospectively matching sequence after it.
    loop
    {
		// Check for the end from the start.  Get out fast, if possible.
		if tame_str.len() <= iwild
		{
			if wild_str.len() > iwild
			{
				while wild_str.as_bytes()[iwild] == '*' as u8
				{
					iwild += 1;
					
					if wild_str.len() <= iwild
					{
						return true;       // "ab" matches "ab*".
					}
				}

			    return false;              // "abcd" doesn't match "abc".
			}
			else
			{
				return true;               // "abc" matches "abc".
			}
		}
		else if wild_str.len() <= iwild
		{
		    return false;                  // "abc" doesn't match "abcd".
		}
		else if wild_str.as_bytes()[iwild] == '*' as u8
		{
			// Got wild: set up for the second loop and skip on down there.
			itame = iwild;

			loop
			{
				iwild += 1;

				if wild_str.len() <= iwild
				{
					return true;               // "abc*" matches "abcd".
				}
				
				if wild_str.as_bytes()[iwild] == '*' as u8
				{
					continue;
				}
				
				break;
			}

			// Search for the next prospective match.
			if wild_str.as_bytes()[iwild] != '?' as u8
			{
				while wild_str.as_bytes()[iwild] != tame_str.as_bytes()[itame]
				{
					itame += 1;

					if tame_str.len() <= itame
					{
						return false;      // "a*bc" doesn't match "ab".
					}
				}
			}

			// Keep fallback positions for retry in case of incomplete match.
			iwild_sequence = iwild;
			itame_sequence = itame;
			break;
		}
		else if wild_str.as_bytes()[iwild] != tame_str.as_bytes()[iwild] && 
				wild_str.as_bytes()[iwild] != '?' as u8
		{
			return false;                  // "abc" doesn't match "abd".
		}

		iwild += 1;                        // Everything's a match, so far.
	}

    // Find any further wildcards and any further matching sequences.
    loop
    {
		if wild_str.len() > iwild && wild_str.as_bytes()[iwild] == '*' as u8
        {
            // Got wild again.
			loop
			{
				iwild += 1;

				if wild_str.len() <= iwild
				{
					return true;           // "ab*c*" matches "abcd".
				}
				
				if wild_str.as_bytes()[iwild] != '*' as u8
				{
					break;
				}
			}

			if tame_str.len() <= itame
            {
                return false;              // "*bcd*" doesn't match "abc".
            }

            // Search for the next prospective match.
            if wild_str.as_bytes()[iwild] != '?' as u8
            {
                while tame_str.len() > itame && 
				      wild_str.as_bytes()[iwild] != tame_str.as_bytes()[itame]
                {
					itame += 1;

                    if tame_str.len() <= itame
                    {
                        return false;      // "a*b*c" doesn't match "ab".
                    }
                }
            }

            // Keep the new fallback positions.
			iwild_sequence = iwild;
			itame_sequence = itame;
        }
		else
		{
            // The equivalent portion of the upper loop is really simple.
            if tame_str.len() <= itame
            {
				if wild_str.len() <= iwild
				{
					return true;           // "*b*c" matches "abc".
				}
			
                return false;              // "*bcd" doesn't match "abc".
            }
			
			if wild_str.len() <= iwild ||
		       wild_str.as_bytes()[iwild] != tame_str.as_bytes()[itame] && 
		       wild_str.as_bytes()[iwild] != '?' as u8
			{
				// A fine time for questions.
				while wild_str.len() > iwild_sequence && 
				      wild_str.as_bytes()[iwild_sequence] == '?' as u8
				{
					iwild_sequence += 1;
					itame_sequence += 1;
				}

				iwild = iwild_sequence;

				// Fall back, but never so far again.
				loop
				{
					itame_sequence += 1;

					if tame_str.len() <= itame_sequence
					{
						if wild_str.len() <= iwild
						{
							return true;   // "*a*b" matches "ab".
						}
						else
						{
							return false;  // "*a*b" doesn't match "ac".
						}
					}

					if wild_str.len() > iwild && wild_str.as_bytes()[iwild] == 
					   tame_str.as_bytes()[itame_sequence]
					{
						break;
					}
				}

	            itame = itame_sequence;
			}
        }

        // Another check for the end, at the end.
        if tame_str.len() <= itame
		{
			if wild_str.len() <= iwild
			{
				return true;           // "*bc" matches "abc".
			}

			return false;              // "*bc" doesn't match "abcd".
		}

        iwild += 1;                    // Everything's still a match.
        itame += 1;
    }
}


// Rust implementation of FastWildCompare(), for UTF-8-encoded content.
//
// Accepts two Box'd slices of 32-bit code points, typically created from 
// Strings, and compares their content.  Accepts '?' as a single-code-point 
// wildcard.  For each '*' wildcard, seeks out a matching sequence of 
// code points beyond it.  Otherwise compares the content a code point at 
// a time.
//
pub fn fast_wild_compare_utf8(
          wild_slice: Box<[char]>, 
          tame_slice: Box<[char]>) -> bool
{
	let mut iwild: usize = 0;  // Index for both inputs in upper loop
	let mut itame: usize;      // Index for tame content, used in lower loop
	let mut iwild_sequence: usize; // Index for prospective match after '*'
	let mut itame_sequence: usize; // Index for match in tame content

    // Find a first wildcard, if one exists, and the beginning of any  
    // prospectively matching sequence after it.
    loop
    {
		// Check for the end from the start.  Get out fast, if possible.
		if tame_slice.len() <= iwild
		{
			if wild_slice.len() > iwild
			{
				while wild_slice[iwild] == '*'
				{
					iwild += 1;
					
					if wild_slice.len() <= iwild
					{
						return true;       // "ab" matches "ab*".
					}
				}

			    return false;              // "abcd" doesn't match "abc".
			}
			else
			{
				return true;               // "abc" matches "abc".
			}
		}
		else if wild_slice.len() <= iwild
		{
		    return false;                  // "abc" doesn't match "abcd".
		}		
		else if wild_slice[iwild] == '*'
		{
			// Got wild: set up for the second loop and skip on down there.
			itame = iwild;

			loop
			{
				iwild += 1;
				
				if wild_slice.len() <= iwild
				{
					return true;           // "abc*" matches "abcd".
				}

				if wild_slice[iwild] == '*'
				{
					continue;
				}
				
				break;
			}

			// Search for the next prospective match.
			if wild_slice[iwild] != '?'
			{
				while wild_slice[iwild] != tame_slice[itame]
				{
					itame += 1;

					if tame_slice.len() <= itame
					{
						return false;      // "a*bc" doesn't match "ab".
					}
				}
			}

			// Keep fallback positions for retry in case of incomplete match.
			iwild_sequence = iwild;
			itame_sequence = itame;
			break;
		}
		else if wild_slice[iwild] != tame_slice[iwild] && 
				wild_slice[iwild] != '?'
		{
			return false;                  // "abc" doesn't match "abd".
		}

		iwild += 1;                        // Everything's a match, so far.
	}

    // Find any further wildcards and any further matching sequences.
    loop
    {
		if wild_slice.len() > iwild && wild_slice[iwild] == '*'
        {
            // Got wild again.
			loop
			{
				iwild += 1;

				if wild_slice.len() <= iwild
				{
					return true;           // "ab*c*" matches "abcd".
				}
				
				if wild_slice[iwild] != '*'
				{
					break;
				}
			}

			if tame_slice.len() <= itame
            {
                return false;              // "*bcd*" doesn't match "abc".
            }

            // Search for the next prospective match.
            if wild_slice[iwild] != '?'
            {
                while tame_slice.len() > itame && 
				      wild_slice[iwild] != tame_slice[itame]
                {
					itame += 1;

                    if tame_slice.len() <= itame
                    {
                        return false;      // "a*b*c" doesn't match "ab".
                    }
                }
            }

            // Keep the new fallback positions.
			iwild_sequence = iwild;
			itame_sequence = itame;
        }
		else
		{
            // The equivalent portion of the upper loop is really simple.
            if tame_slice.len() <= itame
            {
				if wild_slice.len() <= iwild
				{
					return true;           // "*b*c" matches "abc".
				}
			
                return false;              // "*bcd" doesn't match "abc".
            }
			
			if wild_slice.len() <= iwild ||
		       wild_slice[iwild] != tame_slice[itame] && 
		       wild_slice[iwild] != '?'
			{
				// A fine time for questions.
				while wild_slice.len() > iwild_sequence && 
				      wild_slice[iwild_sequence] == '?'
				{
					iwild_sequence += 1;
					itame_sequence += 1;
				}

				iwild = iwild_sequence;

				// Fall back, but never so far again.
				loop
				{
					itame_sequence += 1;

					if tame_slice.len() <= itame_sequence
					{
						if wild_slice.len() <= iwild
						{
							return true;   // "*a*b" matches "ab".
						}
						else
						{
							return false;  // "*a*b" doesn't match "ac".
						}
					}

					if wild_slice.len() > iwild && 
					    wild_slice[iwild] == tame_slice[itame_sequence]
					{
						break;
					}
				}

				itame = itame_sequence;
			}
        }

        // Another check for the end, at the end.
        if tame_slice.len() <= itame
		{
			if wild_slice.len() <= iwild
			{
				return true;           // "*bc" matches "abc".
			}

			return false;              // "*bc" doesn't match "abcd".
		}

        iwild += 1;                    // Everything's still a match.
        itame += 1;	
    }
}


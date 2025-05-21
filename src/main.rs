use std::{collections::HashMap, f32::INFINITY};


/* 
#[derive(PartialEq, Eq, Clone, Debug)]
struct LinkedListNode {
    data:i32,
    next:Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    fn new(data:i32) -> Self {
        LinkedListNode { data: data, next: None }
    }
}

fn gen_linked_list(data: Vec<i32>) -> LinkedListNode {
    let head = LinkedListNode::new(data[0]);
    let mut previous_node : LinkedListNode = head;
    for i in 1..data.len() {
        let lln = LinkedListNode::new(data[i]);
        previous_node.next = Some(Box::new(lln));
    }

    return head;
}
 */

struct Solution {} impl Solution {
    // 1: TwoSum
    #[allow(dead_code)]
    fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1,-1]; // Edge cases, also prevents panics
        for (i,num) in nums.iter().enumerate() {
            let c = target-num; // diff var 
            for j in (i+1)..nums.len() {
                if c == nums[j]{
                    result = vec![i as i32,j as i32];
                    break
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complement: HashMap<i32, i32> = HashMap::new(); // diff_map
        let mut result :Vec<i32> = vec![-1,-1]; // edge cases
    
        for (i,num) in nums.iter().enumerate() {
            let c = target - num; // diff
            if complement.contains_key(num) {
                result = vec![*complement.get(&(num)).unwrap(), i as i32];
                break;
            }
            complement.insert(c, i as i32);
            // insertion is performed after checking if the difference exists
            // because numbers that are exactly half of the target have their indexes
            // returned twice
        }
        return result;
    }

    // Best Time to Buy and Sell Stock I
    // 0ms runtime
    #[allow(dead_code)]
    fn bttbass1_brute(prices: Vec<i32>) -> i32 {
        let mut lowest:f32 = INFINITY;
        let mut highest_profit = 0;
        
        for price in prices {
            if lowest > price as f32 {
                lowest = price as f32
            }
            if price - lowest as i32 > highest_profit {
                highest_profit = price - lowest as i32;
            }
        }

        return highest_profit;
    }

    // 93: Restore IP Addresses
    #[allow(dead_code)]
    /* 
    fn ria_old(s:String) -> Vec<String> {

        fn ip_to_vec(s:&String) -> Vec<u8> {
            let split_ip: Vec<_> = s.split("").collect();
            let mut res : Vec<u8> = vec![];
            for c in split_ip {
                if c != "" {
                    let n : u8 = c.parse().unwrap();
                    res.push(n);
                }
            }
            return res;
        }

        let mut new_octet :bool = true;
        let mut temp_ip : Vec<String> = vec![];
        let mut temp_octet : String = "".to_string();
        let mut result: Vec<String> = vec![];
        let mut restored : bool = false;
        while !restored{
            let split_ip = ip_to_vec(&s);
            for num in split_ip {
                temp_octet.push(num as char);
                let to : u32 = temp_octet.parse().unwrap();
                if to > 255 || temp_octet.len() == 3 {
                    temp_ip.push(num.to_string());
                }
            }
            result.push(temp_ip.join("."));
        }
        return result;
    }
    */
    /* 
    fn restore_ip_addresses(s: String) -> Vec<String> {



        let mut spliced_string : Vec<String> = s.split("").map(|x| x.to_string()).collect();
        let mut solution : Vec<String> = vec![];
        let mut temp_ip : Vec<String> = vec![];

        let string_length = spliced_string.len();

        let mut is_valid_segment = |start:usize, end:usize| -> bool {
            if spliced_string[start] == "0".to_string() && start != end {
                return false
            }
            let num: u32 = s[start..end+1].parse().unwrap();
            return 0 <= num  && num <= 255
        };

        struct BackTrack<'s> { f: &'s dyn Fn(&BackTrack, usize)}



        let mut depth_first_search = BackTrack {f: &|depth_first_search, index : usize | 
            if index >= spliced_string.len() && temp_ip.len() == 4 {
                solution.push(temp_ip.join("."));
                return
            } else           
            if index >= string_length || temp_ip.len() >= 4 {
                return
            } else {

                for i in index..(f64::min((index + 3) as f64, string_length as f64) as usize) {
                    if is_valid_segment(index, i) {
                        temp_ip.push(spliced_string[index..i+1].join(""));
                        (depth_first_search.f)(depth_first_search, i + 1);
                        temp_ip.pop();
                    }
                }
            } 

        };

        (depth_first_search.f)(&depth_first_search, 0);
        return solution
    }
    */

    // RIA using Depth First Search (DFS) and Backtracking
    // 0ms runtime, 2.1 MB memory usage
    fn restore_ip_addresses(s: String) -> Vec<String> {
        // BEcause rust is relaly annoying about bowrowing we define
        // an environemnt for the recursive DFS fucntion
        struct BackTrackEnv {
            solutions : Vec<String>,    // Store potential solutions
            temp_ip : Vec<String>,      // The current Ip candidate being tested
            s : Vec<String>,            // numeric String to Vec<String>
            string_length : usize,      // Length of the String
        }

        // Defining our recursive function 
        // env : our backtracking mutable eenvironment
        // index : root node of our search
        fn depth_first_search(env: &mut BackTrackEnv, index : usize) {
            // checking if wee have a valid candidate and pushing to solutions
            if index >= env.string_length && env.temp_ip.len() == 4 {
                env.solutions.push(env.temp_ip.join("."));
                return
            };        
            // returning if invalid candidate
            if index >= env.string_length || env.temp_ip.len() >= 4 {
                return
            };
            // checking within the 3 char segment for a valid octet between 0 and 255
            for i in index..(index + 3).min(env.string_length) {
                // segment to check
                let segment = &env.s[index..=i];
            
                // checking the segment, if segment is within the range,
                // push to temp_ip
                if !(segment[0] == "0" && segment.len() > 1) {
                    let num: u32 = segment.join("").parse().unwrap();
                    if num <= 255 {
                        env.temp_ip.push(segment.join(""));
                        depth_first_search(env, i + 1);
                        env.temp_ip.pop();
                    }
                }
            }
        }

        // setting values for our environment
        let mut btenv = BackTrackEnv {
            solutions : vec![], // empty
            temp_ip : vec![],   // empty
            s : s.chars().map(|x| x.to_string()).collect(), // numeric String to Vec<STring> converter
            string_length : s.len() // leng of String
        };
        // initialising our recursive function
        // and passing our created environment
        depth_first_search(&mut btenv, 0);
        // return the solutions inside our backtracking environemnt
        return btenv.solutions;
    }


    // 65 : Is valid number
    // 0ms runtime, 2.08 MB memory
    #[allow(dead_code)]
    /* 
    fn valid_number(s:String) -> bool {

        let mut is_digit = |s: char| -> bool {
            
            match s {
                "0" => {
                    return true;
                },
                "1" => {
                    return true;
                },
                "2" => {
                    return true;
                },
                "3" => {
                    return true;
                },
                "4" => {
                    return true;
                },
                "5" => {
                    return true;
                },
                "6" => {
                    return true;
                },
                "7" => {
                    return true;
                },
                "8" => {
                    return true;
                },
                "9" => {
                    return true;
                },
                _ => {
                    return false;
                }
            }
        };

        let mut is_integer = |s:String| -> bool {
            let mut valid_int : bool = true;

            let split_str = s.chars();

            match split_str.next() {
                "-" => {
                    if is_digit(split_str.next()) {

                    }
                },
                "+" => {

                },
                _ => {

                }
            }

            return valid_int;
        }


        return true
    }
 */

    // 
    fn valid_number(s:String) -> bool {
        let mut in_exponent         : bool = false;
        let mut passed_sign         : bool = false;
        let mut passed_dot          : bool = false;
        let mut last_char_digit     : bool = false;
        let mut prev_character            = "";

        let s:Vec<String> = s.chars().map(|x| x.to_string()).collect();
        for i in 0..s.len() {
            let character = s[i].as_str();
            match character {
                "-" | "+" => {
                    if passed_sign || last_char_digit || i == s.len()-1 || prev_character == "." || (in_exponent && ( character == "+" || character == "-" ) && !(prev_character == "e" || prev_character == "E")) {
                        return false;
                    }
                    passed_sign = true;
                },
                "1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"0" => {
                    last_char_digit = true;
                },
                "e" | "E" => {
                    if i == s.len()-1 || !last_char_digit || in_exponent {
                        return false;
                    } else {
                        in_exponent = true;
                        passed_sign = false;
                        last_char_digit = false;
                    }

                },
                "." => {
                    if in_exponent || passed_dot || (!last_char_digit && i == s.len() -1) {
                        return false
                    }
                    passed_dot = true;
                    //last_char_digit = true;
                },
                _ => {return false}
            }
            prev_character = character;
        }

        return true
    }




}


fn main() {
    // TwoSum
/*     let nums: Vec<i32> = vec![0,61,37,843,6,25,0,13,73,64];
    let target = 0;
    println!("hashmap : {:?}", Solution::two_sum_hash(nums.clone(), target));
    println!("brute   : {:?}", Solution::two_sum_brute(nums.clone(), target)); */


    // Best Time to Buy and Sell Stocks 1
    /* 
    let prices: Vec<i32> = vec![7,5,1,4,6,3];
    let prices_0: Vec<i32> = vec![7,6,4,3,1];
    let prices_not_smallest: Vec<i32> = vec![2,4,1];
    println!("{}", Solution::bttbass1_brute(prices));
    println!("{}", Solution::bttbass1_brute(prices_0));
    println!("{}", Solution::bttbass1_brute(prices_not_smallest));
     */

    //println!("{:?}", gen_linked_list(vec![1,2]))


    // Restore IP Adresses
/*     let inputs : Vec<&str> = vec![
        "25525511135",
        "0000",
        "101023",
        "2302401",
        "3957683",
        "252535525"
    ];
    for input in inputs {
        println!("{:?}", Solution::restore_ip_addresses(input.to_string()))
    } */


    // Valid Number

    let inputs : Vec<&str> = vec![
        // all invalid
"abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", "4+1", "4-1","-.7e+-0435", "5+", "5-", ".-4", "092e359-2",
        // all valid
"2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789", "0", "-1E+3", "32.e-80123"
    ];

    for input in inputs {
        println!("{} : {}", input, Solution::valid_number(input.to_string()))
    }

}



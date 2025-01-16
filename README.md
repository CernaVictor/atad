# atad

repository for atad lab project

# solve me first

solveMeFirst function:

- The solve_me_first function takes two integers a and b, and returns their sum.

# simple array sum

simpleArraySum function:

- ar.iter().sum(): This line uses Rust's iterator methods to sum up all elements in the slice ar. The iter() method creates an iterator over the array, and sum() adds up the elements. The result is returned as an i32 (the type of the array elements).

# compare the triplets

compareTriplets function:

- The function takes two slices a and b as input (ratings of Alice and Bob).
- It initializes a scores vector to keep track of Alice's and Bob's points.
- It then loops through the ratings (using a.len() to ensure the loop runs for all three ratings) and compares each corresponding element:
  - If Alice's rating is higher, she gets a point.
  - If Bob's rating is higher, he gets a point.
  - If their ratings are equal, no points are awarded.
- Finally, it returns the scores vector.

# diagonal difference

diagonalDifference function:

- Input: A 2D array arr of integers (matrix).
- Logic:
  - We initialize two variables: primary_diagonal_sum and secondary_diagonal_sum to 0.
  - The matrix is square, so we only need to iterate through the rows (or columns) using a loop from 0 to n - 1.
  - For each row i:
    - The primary diagonal element is arr[i][i].
    - The secondary diagonal element is arr[i][n - 1 - i] (this is the element from the other diagonal).
  - After the loop, we compute the absolute difference between the two sums and return the result.

# plus minus

plusMinus function:

- The function takes a slice arr of integers as its parameter.
- We calculate the total number of elements n using arr.len(), which gives the length of the array, then cast it to a f64 for accurate division.
- We initialize three counters: positive_count, negative_count, and zero_count, all set to 0.
- We loop through each element in arr:
  - If the element is positive, increment positive_count.
  - If the element is negative, increment negative_count.
  - If the element is zero, increment zero_count.
- After counting the values, we compute the ratios by dividing the counts by the total number of elements n.
- Finally, we print the three ratios with six decimal places using println!("{:.6}", value).

# staircase

staircase function:

- The function takes an integer n representing the size of the staircase.
- We use a for loop with the range 1..=n (which includes n) to iterate through each row of the staircase.
- For each row i:
  - We calculate the number of spaces needed as n - i.
  - We calculate the number of # symbols as i.
- The spaces and # symbols are generated using the repeat() method, which creates a string with the required number of characters.
- The final row is printed by concatenating spaces and # symbols and then printing the result with println!().

# birthday cake candles

birthdayCakeCandles function:

- Step 1: We use candles.iter().max() to get the tallest candle in the list. This returns an Option<&i32>, so we unwrap() it to get the value (assuming the list is non-empty).
- Step 2: Using filter(|&&candle| candle == max_height) we iterate over all the candles and count how many match the maximum height. This gives us the total number of tallest candles.

# grading students

gradingStudents function:

- We use .iter().map() to transform the grades vector into a new one by applying the rounding rules.
- For each grade:
  - If it’s below 38, it’s returned unchanged.
  - Otherwise, we compute the next multiple of 5 (next_multiple_of_5 = (grade / 5 + 1) \* 5).
  - If the difference between the next multiple of 5 and the grade is less than 5, the grade is rounded to that multiple.
  - If not, the grade remains unchanged.

# apples and oranges

countApplesAndOranges Function:

- The function takes the starting and ending points of Sam's house (s and t), the positions of the apple and orange trees (a and b), and two slices of integers representing the distances each apple and orange falls.
- It calculates where each apple and orange lands by adding their respective distances to the positions of the apple and orange trees (a + apple for apples, b + orange for oranges).
- It then checks whether the landing position falls within the range [s, t] and counts the number of apples and oranges that land within this range.
- The results (counts of apples and oranges) are printed on two separate lines.

# breaking the records

breakingRecords function:

- This function takes a slice of scores (&[i32]), initializes the max_score and min_score to the first score, and then iterates over the rest of the scores.
- It keeps track of how many times the max_score and min_score are broken, incrementing the respective counters (max_breaks and min_breaks) whenever a new record is set.

# bill division

bonAppetit function:

- This function takes the bill array, k (the index of the item Anna didn’t eat), and b (the amount Brian charged Anna).

- It calculates the total cost of the items Anna actually ate, then splits that total between Anna and Brian.

- Finally, it checks if Brian charged Anna correctly. If he did, it prints "Bon Appetit". If not, it prints the difference (how much Brian owes Anna).

# extra long factorial

extraLongFactorials:

- This function calculates the factorial of n by iteratively multiplying the current factorial by every integer from 2 to n. The result is stored as a vector of digits (Vec<u8>), where each element represents a digit of the number, stored in reverse order (to make multiplication easier).

Big Integer Representation:

- We use a vector result to store the digits of the large number.
- Initially, result = vec![1] because the factorial of 0 and 1 is 1.
- The multiplication logic is implemented manually to handle numbers that don't fit into regular integer types (i.e., numbers larger than i32).

multiply_bigint:

- This helper function takes a mutable reference to the bigint vector and an integer num, and multiplies the number stored in the vector by num.
- The multiplication is done digit by digit, from the least significant digit (rightmost) to the most significant, handling carry values.

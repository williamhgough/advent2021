package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("./input.txt")
	if err != nil {
		fmt.Println(err)
	}

	numIncreased, prevNum := 0, 0
	numbers := []int{}
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			fmt.Println(err)
		}

		if num > prevNum && prevNum != 0 {
			numIncreased++
		}

		prevNum = num
		numbers = append(numbers, num)
	}

	fmt.Printf("Number increased: %d\n", numIncreased)

	numIncreased = 0
	prevSum := 0
	for i := 0; i < len(numbers)-2; i++ {
		num := numbers[i] + numbers[i+1] + numbers[i+2]

		if num > prevSum && prevSum != 0 {
			numIncreased += 1
		}

		prevSum = num
	}
	fmt.Printf("Num of larger sums: %d\n", numIncreased)
}

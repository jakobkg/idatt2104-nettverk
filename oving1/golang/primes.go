package main

import (
	"flag"
	"fmt"
	"math"
	"os"
	"sort"
	"sync"
)

func primeRoutine(numbers []int, c chan int, g *sync.WaitGroup) {
	for _, number := range numbers {
		if checkPrime(number) {
			c <- number
		}
	}
	defer g.Done()
}

func checkPrime(number int) bool {
	if number < 2 || (number > 2 && number%2 == 0) {
		return false
	}

	root := int(math.Sqrt(float64(number)))

	for factor := 3; factor <= root; factor++ {
		if number%factor == 0 {
			return false
		}
	}

	return true
}

func main() {
	var from int
	var to int
	var threadcount int

	flag.IntVar(&from, "fra", -1, "Første tall i rekken som skal sjekkes for primtall")
	flag.IntVar(&to, "til", -1, "Siste tall i rekken som skal sjekkes for primtall")
	flag.IntVar(&threadcount, "n", -1, "Antall tråder som skal brukes for å lete etter primtall")

	flag.Parse()

	if from == -1 || to == -1 || threadcount == -1 {
		fmt.Println("Mangler argument!")
		os.Exit(1)
	}

	c := make(chan int, to-from)

	numbers := [][]int{}

	for len(numbers) < threadcount {
		numbers = append(numbers, []int{})
	}

	thread := 0

	for i := from; i <= to; i++ {
		numbers[thread] = append(numbers[thread], i)
		thread++
		if thread == threadcount {
			thread = 0
		}
	}

	var waiter sync.WaitGroup

	for _, list := range numbers {
		waiter.Add(1)
		go primeRoutine(list, c, &waiter)
	}

	waiter.Wait()

	close(c)

	primes := []int{}

	for number := range c {
		primes = append(primes, number)
	}

	sort.Ints(primes)

	fmt.Println(primes)
}

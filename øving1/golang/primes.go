package main

import (
	"flag"
	"fmt"
	"math"
	"os"
	"sort"
	"sync"
)

func primeRoutine(numbers []int, c chan int, semaphore *sync.WaitGroup) {
	for _, number := range numbers {
		if checkPrime(number) {
			c <- number
		}
	}

	semaphore.Done()
}

func checkPrime(number int) bool {
	// Hvis et tall er mindre enn 2, eller er et partall større enn 2, er det ikke et primtall
	if number < 2 || (number > 2 && number%2 == 0) {
		return false
	}

	// Største mulige faktor i et tall er rota til tallet, estimerer denne
	root := int(math.Sqrt(float64(number)))

	// Sjekk om tallet er delelig på noen av oddetallene fra og med 3 til og med rota
	for factor := 3; factor <= root; factor += 2 {
		// Er det delelig såer det ikke et primtall
		if number%factor == 0 {
			return false
		}
	}

	// Om vi ikke fant noen faktorer må tallet være et primtall!
	return true
}

func main() {
	// Inputs
	var from int
	var to int
	var threadcount int

	// Koble inputs til terminal-argumenter
	flag.IntVar(&from, "fra", -1, "Første tall i rekken som skal sjekkes for primtall")
	flag.IntVar(&to, "til", -1, "Siste tall i rekken som skal sjekkes for primtall")
	flag.IntVar(&threadcount, "n", -1, "Antall tråder som skal brukes for å lete etter primtall")

	flag.Parse()

	// Sjekk at vi faktisk fikk inputs
	if from == -1 || to == -1 || threadcount == -1 {
		fmt.Println("Mangler argument!")
		os.Exit(1)
	}

	// Opprett kanal for å sende primtall som er funnet fra bi-tråder til hovedtråden
	prime_channel := make(chan int, to-from)

	// Liste med lister av tall, der hver liste av tall skal sendes til en goroutine for å sjekkes for primtall
	numbers := [][]int{}

	for len(numbers) < threadcount {
		numbers = append(numbers, []int{})
	}

	thread := 0

	// Fyll inn tall fra og med "from" til og med "to" number ovennevnte lister
	for number := from; number <= to; number++ {
		// Ikke ta med partall (unntatt 2) for videre sjekking
		if number%2 == 1 || number == 2 {
			numbers[thread] = append(numbers[thread], number)
			thread++

			if thread == threadcount {
				thread = 0
			}
		}
	}

	// Opprett en semafor som brukes for å vente til alle bi-trådene er ferdig med arbeidet sitt
	var semaphore sync.WaitGroup

	// Kjør alle goroutines (tråder)
	for _, list := range numbers {
		semaphore.Add(1)
		go primeRoutine(list, prime_channel, &semaphore)
	}

	// Vent til alle sammen er ferdige
	semaphore.Wait()

	// Lukk kanalen de har brukt til å sende primtall
	close(prime_channel)

	primes := []int{}

	// Tøm bufferen fra kanalen inn i listen
	for number := range prime_channel {
		primes = append(primes, number)
	}

	// Sorter og skriv ut primtallene
	sort.Ints(primes)

	fmt.Println(primes)
}

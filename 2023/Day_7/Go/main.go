package main

import (
	"bufio"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	hands := make([]*Hand, 0)
	ranks := len(lines)
	scoreMap := map[string]int{
		"A": 14,
		"K": 13,
		"Q": 12,
		"J": 11,
		"T": 10,
		"9": 9,
		"8": 8,
		"7": 7,
		"6": 6,
		"5": 5,
		"4": 4,
		"3": 3,
		"2": 2,
	}
	scoreMapP2 := map[string]int{
		"A": 14,
		"K": 13,
		"Q": 12,
		"T": 10,
		"9": 9,
		"8": 8,
		"7": 7,
		"6": 6,
		"5": 5,
		"4": 4,
		"3": 3,
		"2": 2,
		"J": 1,
	}

	for _, line := range lines {
		raw := strings.Fields(line)
		bid, _ := strconv.Atoi(raw[1])
		cards := strings.Split(raw[0], "")
		hand := NewHand(cards, bid)
		hands = append(hands, hand)
	}

	// for _, hand := range hands {
	// 	println(hand.typ3.String())
	// }

	sort.Slice(hands, func(i, j int) bool {
		if int(hands[i].typ3) == int(hands[j].typ3) {
			for ii := 0; ii < len(hands[ii].cards); ii++ {
				if hands[i].cards[ii] == hands[j].cards[ii] {
					continue
				}
				return scoreMap[hands[i].cards[ii]] > scoreMap[hands[j].cards[ii]]
			}
		}
		return int(hands[i].typ3) > int(hands[j].typ3)
	})

	// println("-------------------")
	// for i, hand := range hands {
	// 	println(hand.typ3.String(), " ", hand.cards.String(), " ", hand.bid, " * ", (ranks - i))
	// }
	// Part 1
	result := 0
	for i, hand := range hands {
		result += hand.bid * (ranks - i)
	}
	println(result)

	// Part 2
	hands = make([]*Hand, 0)
	for _, line := range lines {
		raw := strings.Fields(line)
		bid, _ := strconv.Atoi(raw[1])
		cards := strings.Split(raw[0], "")
		hand := NewBestHand(cards, bid)
		hands = append(hands, hand)
	}

	sort.Slice(hands, func(i, j int) bool {
		if int(hands[i].typ3) == int(hands[j].typ3) {
			for ii := 0; ii < len(hands[ii].cards); ii++ {
				if hands[i].cards[ii] == hands[j].cards[ii] {
					continue
				}
				return scoreMapP2[hands[i].cards[ii]] > scoreMapP2[hands[j].cards[ii]]
			}
		}
		return int(hands[i].typ3) > int(hands[j].typ3)
	})
	// println("-------------------")
	// for i, hand := range hands {
	// 	println(hand.typ3.String(), " ", hand.cards.String(), " ", hand.bid, " * ", (ranks - i))
	// }
	result = 0
	for i, hand := range hands {
		result += hand.bid * (ranks - i)
	}
	println(result)

}

func strToIntArray(str string) []int {
	vals := strings.Fields(str)
	var res []int
	for i, v := range vals {
		if i == 0 {
			continue
		}
		val, _ := strconv.Atoi(v)
		res = append(res, val)
	}
	return res
}

func NewHand(cards []string, bid int) *Hand {
	return &Hand{cards: cards, bid: bid, typ3: getHandType(cards)}
}

func NewBestHand(cards []string, bid int) *Hand {
	return &Hand{cards: cards, bid: bid, typ3: getBestHandType(cards)}
}

func getHandType(cards []string) HandType {
	cardMap := make(map[string]int)
	for _, card := range cards {
		cardMap[card]++
	}
	// check five of a kind
	if cardMap[cards[0]] == 5 {
		return FiveOfAKind
	}
	// check four of a kind three of a kind
	if len(cardMap) == 2 {
		for _, v := range cardMap {
			if v == 4 || v == 1 {
				return FourOfAKind
			}
			if v == 2 || v == 3 {
				return FullHouse
			}
		}
	}
	// check three of a kind and two pairs
	if len(cardMap) == 3 {
		for _, v := range cardMap {
			if v == 3 {
				return ThreeOfAKind
			}
			if v == 2 {
				return TwoPairs
			}
		}
	}
	// check one pair
	if len(cardMap) == 4 {
		return OnePair
	}
	return HighCard
}

func getBestHandType(cards []string) HandType {
	cardMap := make(map[string]int)
	for _, card := range cards {
		cardMap[card]++
	}
	jokers := 0
	// Check for Jokers
	if cardMap["J"] > 0 {
		jokers = cardMap["J"]
		if jokers == 5 || jokers == 4 {
			return FiveOfAKind
		}
		if jokers == 3 {
			if len(cardMap) == 2 {
				// XX JJJ
				return FiveOfAKind
			}
			if len(cardMap) == 3 {
				// X X JJJ
				return FourOfAKind
			}
		}
		if jokers == 2 {
			if len(cardMap) == 2 {
				// XXX JJ
				return FiveOfAKind
			}
			if len(cardMap) == 3 {
				// XX X JJ
				return FourOfAKind
			}
			if len(cardMap) == 4 {
				// X X X JJ
				return ThreeOfAKind
			}
		}
		if jokers == 1 {
			if len(cardMap) == 2 {
				// XXXX J
				return FiveOfAKind
			}
			if len(cardMap) == 3 {

				for _, v := range cardMap {
					if v == 3 {
						// XXX X J - FourOfAKind
						return FourOfAKind
					}
					if v == 2 {
						// XX XX J - FullHouse
						return FullHouse
					}
				}

				return ThreeOfAKind
			}
			if len(cardMap) == 4 {
				// XX X X J
				return ThreeOfAKind
			}
			if len(cardMap) == 5 {
				// X X X X J
				return OnePair
			}
		}
	}

	// check five of a kind
	if cardMap[cards[0]] == 5 {
		return FiveOfAKind
	}
	// check four of a kind three of a kind
	if len(cardMap) == 2 {
		for _, v := range cardMap {
			if v == 4 || v == 1 {
				return FourOfAKind
			}
			if v == 2 || v == 3 {
				return FullHouse
			}
		}
	}
	// check three of a kind and two pairs
	if len(cardMap) == 3 {
		for _, v := range cardMap {
			if v == 3 {
				return ThreeOfAKind
			}
			if v == 2 {
				return TwoPairs
			}
		}
	}
	// check one pair
	if len(cardMap) == 4 {
		return OnePair
	}
	return HighCard
}

type ListOfCards []string

type Hand struct {
	cards ListOfCards
	bid   int
	typ3  HandType
}

type HandType int

const (
	HighCard HandType = iota
	OnePair
	TwoPairs
	ThreeOfAKind
	FullHouse
	FourOfAKind
	FiveOfAKind
)

func (l ListOfCards) String() string {
	result := ""
	for _, v := range l {
		result += v
	}

	return result
}

func (h HandType) String() string {
	switch h {
	case FiveOfAKind:
		return "FiveOfAKind"
	case FourOfAKind:
		return "FourOfAKind"
	case ThreeOfAKind:
		return "ThreeOfAKind"
	case FullHouse:
		return "FullHouse"
	case TwoPairs:
		return "TwoPairs"
	case OnePair:
		return "OnePair"
	case HighCard:
		return "HighCard"
	default:
		return "Unknown"
	}
}

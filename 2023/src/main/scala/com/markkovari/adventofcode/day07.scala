package com.markkovari.adventofcode.day07

import scala.math.Ordering.Implicits.*

object DataDefs:
  enum Card:
    case Num(n: Int)
    case Ten
    case Jack
    case Queen
    case King
    case Ace
    lazy val value: Int = this match
      case Ace    => 14
      case King   => 13
      case Queen  => 12
      case Jack   => 11
      case Ten    => 10
      case Num(n) => n
    lazy val jokerValue: Int = this match
      case Ace    => 14
      case King   => 13
      case Queen  => 12
      case Ten    => 10
      case Num(n) => n
      case Jack   => 1

  given Ordering[Card] with
    def compare(x: Card, y: Card): Int = x.value - y.value

  enum Rank:
    case HighCard, OnePair, TwoPairs, ThreeOfAKind, FullHouse, FourOfAKind,
      FiveOfAKind

  import Rank.*

  given Ordering[Rank] with // part 1
    def compare(x: Rank, y: Rank): Int = x.ordinal compare y.ordinal

  case class Hand(cards: Seq[Card]):
    private lazy val partition = cards
      .groupMapReduce(identity)(_ => 1)(_ + _)
      .values
      .toList
      .sorted
    lazy val rank: Rank = partition match
      case List(5)          => FiveOfAKind
      case List(1, 4)       => FourOfAKind
      case List(2, 3)       => FullHouse
      case List(1, 1, 3)    => ThreeOfAKind
      case List(1, 2, 2)    => TwoPairs
      case List(1, 1, 1, 2) => OnePair
      case _                => HighCard

    private lazy val isAllJokers = cards.forall(_ == Card.Jack)
    private lazy val indexedCards = cards.zipWithIndex
    private lazy val (jokers, others) =
      indexedCards.partition((card, _) => card == Card.Jack)
    private lazy val nonJokers = others.map(_._1).distinct
    private lazy val substitutes =
      for nonJoker <- nonJokers
      yield jokers.map((_, index) => (nonJoker, index))
    private lazy val subbedHands =
      for sub <- substitutes
      yield Hand((others ++ sub).sortBy(_._2).map(_._1))
    lazy val jokerRank: Rank =
      if isAllJokers then FiveOfAKind else subbedHands.maxBy(_.rank).rank

  given Ordering[Hand] with
    def compare(x: Hand, y: Hand): Int =
      Ordering[Rank].compare(x.rank, y.rank) match
        case 0 => Ordering[Seq[Card]].compare(x.cards, y.cards)
        case n => n

  case class Bid(hand: Hand, bid: Int)

  given Ordering[Bid] with
    def compare(x: Bid, y: Bid): Int = Ordering[Hand].compare(x.hand, y.hand)

  object Joker:
    given Ordering[Card] with
      def compare(x: Card, y: Card): Int = x.jokerValue - y.jokerValue

    given Ordering[Hand] with
      def compare(x: Hand, y: Hand): Int =
        Ordering[Rank].compare(x.jokerRank, y.jokerRank) match
          case 0 => Ordering[Seq[Card]].compare(x.cards, y.cards)
          case n => n

    given Ordering[Bid] with
      def compare(x: Bid, y: Bid): Int = Ordering[Hand].compare(x.hand, y.hand)

  object Parsing:

    import DataDefs.*
    import Card.*

    extension (char: Char)
      def toCard: Card = char match
        case 'A' => Ace
        case 'K' => King
        case 'Q' => Queen
        case 'J' => Jack
        case 'T' => Ten
        case n   => Num(n.asDigit)

    def parseHand(hand: String): Hand = Hand(hand.map(_.toCard))

    def lineToBid(line: String): Bid = line match
      case s"$hand $bid" => Bid(parseHand(hand), bid.toInt)

object Solving:

  import DataDefs.*

  def solve(lines: Seq[String])(using Ordering[Bid]): Long = lines
    .map(Parsing.lineToBid)
    .sorted
    .zipWithIndex
    .map((bid, index) => bid.bid * (index + 1))
    .sum

def part1(input: String): Long = {
  Solving.solve(input.linesIterator.toSeq)
}

def part2(input: String): Long = {
  import DataDefs.Joker.given
  Solving.solve(input.linesIterator.toSeq)
}

package com.markkovari.adventofcode.day09

type History = List[Long]

def parse(input: String): Seq[Seq[Int]] =
  input.linesIterator
    .map(_.split(' ').map(_.toInt).toSeq)
    .toSeq

def extrapolate(xs: Seq[Int]): Int =
  if xs.forall(_ == xs.head)
  then xs.head
  else
    xs.last + extrapolate(
      xs.tail
        .lazyZip(xs)
        .map(_ - _)
    )

def part1(input: String): Int =
  parse(input)
    .map(extrapolate)
    .sum

def part2(input: String): Int =
  parse(input)
    .map(_.reverse)
    .map(extrapolate)
    .sum

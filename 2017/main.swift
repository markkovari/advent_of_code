import Foundation

var content = try! String(contentsOfFile: "./inputs/1/prod.data", encoding: String.Encoding.utf8)

print(content)
print(sumRecurringDigits(of: content))
print(inverseCaptcha(of: content))

func sumRecurringDigits(of: String) -> Int {

  var sum = 0
  var last_digit = 0
  var first_digit = 0
  var first_digit_found = false

  for digit in of {
    let digit = Int(String(digit))!

    if !first_digit_found {
      first_digit = digit
      first_digit_found = true
    }

    if last_digit == digit {
      sum += digit
    }

    last_digit = digit
  }

  if last_digit == first_digit {
    sum += last_digit
  }

  return sum
}

public func inverseCaptcha(of: String) -> Int {
  let numbers = of.map { Int(String($0))! }

  return numbers.enumerated().reduce(
    0,
    { (memo, element) in
      let (index, number) = element
      let nextNumber = numbers[(index + (numbers.count / 2)) % numbers.count]

      return number == nextNumber ? (memo + number) : (memo)
    })
}

use crate::day11::{Data, Input, Op::*, Oper};
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/11/input.txt");

pub fn read() -> Input {
    let op_check = parser!({"+"=> Add,"*"=>Multiply});
    let p = parser!(
        repeat_sep(
            "Monkey " (id: u32) ":"
            "\n  Starting items: " (items:repeat_sep(u128, ", "))
            "\n  Operation: new = old " (op:{ "* old" => Oper{operation: Power, adden:2}, (operation: op_check) " " (adden: u128)=> Oper{operation: operation, adden:adden}})
            "\n  Test: divisible by " (test: u128)
            "\n    If true: throw to monkey " (test_true: u32)
            "\n    If false: throw to monkey " (test_false: u32)
			=>Data {id:id, items:items, operation:op,test:test,test_true:test_true,test_false:test_false},"\n\n"
        ) 
    );

    p.parse(INPUT).unwrap()
}

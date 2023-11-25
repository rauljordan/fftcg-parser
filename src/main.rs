#![allow(dead_code)]
mod parsers_from_scratch;

use lazy_static::lazy_static;
use nom::{branch::*, bytes::complete::tag, character::complete::*, sequence::*, *};
use serde::Deserialize;

fn main() {
    for card in DATA.cards.iter() {
        if card.text_en.is_some() {
            println!("{:?}\n", card.text_en);
        }
    }
    match when("when seymour enters the field, draw 10 cards") {
        Ok((_, item)) => println!("Parsed: {:?}", item),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[derive(Debug)]
struct WhenStatement {
    condition: (Occurence, Predicate),
    action: String,
    consequence: Consequence,
}

#[derive(Debug, PartialEq)]
enum Occurence {
    Name { name: String },
}

#[derive(Debug, PartialEq)]
enum Predicate {
    EntersTheField,
}

#[derive(Debug, PartialEq)]
enum Consequence {
    Draw { amount: u8 },
}

fn parse_occurence(input: &str) -> IResult<&str, Occurence> {
    alt((tag("seymour").map(|name: &str| Occurence::Name {
        name: name.to_string(),
    }),))(input)
}

fn parse_predicate(input: &str) -> IResult<&str, Predicate> {
    alt((tag("enters the field").map(|_| Predicate::EntersTheField),))(input)
}

fn parse_consequence(input: &str) -> IResult<&str, Consequence> {
    alt((draw.map(|amount| Consequence::Draw { amount }),))(input)
}

fn draw(input: &str) -> IResult<&str, u8> {
    let (input, (_, _, num_cards, _, _)) =
        tuple((tag("draw"), space1, digit1, space1, tag("cards")))(input)?;

    let num: Result<u8, _> = num_cards.parse();

    Ok((input, num.unwrap()))
}

fn when(input: &str) -> IResult<&str, WhenStatement> {
    let (input, (_, occurence, _, predicate, _, consequence)) = tuple((
        tag("when "),
        parse_occurence,
        space1,
        parse_predicate,
        tag(", "),
        parse_consequence,
    ))(input)?;

    Ok((
        input,
        WhenStatement {
            condition: (occurence, predicate),
            action: "".to_string(),
            consequence: consequence,
        },
    ))
}

fn duller() {
    let _ = "《ダル》";
    let _ = "Haste First Strike Brave[[br]]";
    let _ = "when _ enters the field, ___. ___"; // draw 1 card, break it.
    let _ = "remove them from the game";
    let _ = "deal ___ damage to ____";
    let _ = "all the forwards your opponent controls";
    let _ = "you may play any number of";
    let _ = "up to 3 forwards among all break zones";
    let _ = "is put from the field into the break zone";
    let _ = "select up to _ of the _ following actions";
    let _ = "select _ of the following _ actions";
    let _ = "reveal the top _ cards of your deck";
    let _ = "dull them and freeze them";
    let _ = "deal it _ damage";
    let _ = "is put from the field into the break zone";
    let _ = "your opponent selects _ they control";
    let _ = "deals damage to your opponent";
    let _ = "you may play _ from your hand onto the field dull"; // 1 fire backup
    let _ = "the _ you control cannot be broken by your opponent's Summons or abilities";
    let _ = "[Card Name (Brynhildr)]";
    let _ = "if you have cast _ this turn";
    let _ = "remove them from the game instead";
    let _ = "cannot be blocked";
    let _ = "by a forward";
    let _ = "by a forward of cost _ or more";
    let _ = "it cannot be broken this turn";
    let _ = "the _ you control gain _";
    let _ = "deals you _ point of damage";
    let _ = "put it into the break zone";
    let _ = "dull it";
    let _ = "discard _ card from your hand";
    let _ = "if you control _ or more _";
    let _ = "then";
    let _ = "reveal the top _ cards of your deck";
    let _ = "in your break zone";
    let _ = "add _ among them to your hand";
    let _ = "return the other cards to the bottom of your deck in any order";
    let _ = "can form a party with _ of any element";
    let _ = "select up to _ of the _ following actions instead";
    let _ = "when _ attacks";
    let _ = "freeze it";
    let _ = "you may pay _";
    let _ = "activate them";
    let _ = "activate it";
    let _ = "at the beginning of the Attack Phase during each of your turns";
    let _ = "has entered your field this turn";
    let _ = "it loses _";
    let _ = "return them to their owners' hands";
    let _ = "it gains _ and _";
    let _ = "choose 1 forward of cost 3 or less your opponent controls";
    let _ = "at the end of each of your turns";
    let _ = "draw _ cards";
    let _ = "put _ into the break zone";
    let _ = "if you control _ or more _ backups";
    let _ = "the cost required to cast _ is reduced by _ for each _ you control";
    let _ = "dull all the forwards your opponent controls";
    let _ = "leaves the field";
    let _ = "enters the field";
    let _ = "enters the field or attacks";
    let _ = "you may search for";
    let _ = "and add it to your hand";
    let _ = "you may discard 1 Multi-Element card";
    let _ = "your opponent discards _ card";
    let _ = "_ forward and _ backup";
    let _ = "_ character";
    let _ = "the forwards other than _ you control gain _";
    let _ = "when _ is chosen by your opponent's ability";
    let _ = "when _ you cast a summon";
    let _ = "when _ ";
    let _ = "choose _";
    let _ = "you may discard _";
    let _ = "you may discard _";
}

lazy_static! {
    pub static ref DATA: Data = {
        let cards = std::fs::read_to_string("data/cards.json").unwrap();
        let data: Data = serde_json::from_str(&cards).unwrap();
        data
    };
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub cards: Vec<JsonCard>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonCard {
    pub id: i64,
    pub category_1: Option<String>,
    pub category_2: Option<String>,
    pub code: String,
    pub cost: String,
    pub element: Option<Vec<String>>,
    pub images: Images,
    pub ex_burst: String,
    pub job_en: String,
    pub name_en: String,
    pub text_en: Option<String>,
    pub multicard: Option<String>,
    pub power: Option<String>,
    pub rarity: Option<String>,
    pub set: String,
    pub type_en: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Images {
    pub full: Vec<String>,
    pub thumbs: Vec<String>,
}

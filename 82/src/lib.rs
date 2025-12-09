use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};

// --- Global Helpers (No Dependencies) ---

// H(m: bytes) -> bytes
fn h(m: &[u8]) -> Vec<u8> {

// bit(h: bytes, i: int) -> int
fn bit(h_val: &[u8], i: usize) -> u8 {
    
}

// expmod(b:int,e:int,m:int) -> int
pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {

// inv(x:int, q: &BigInt) -> int
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {

// xrecover helper (nested for local use in setup and decode)
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {

// --- Core Functions (Require Constants) ---

fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {

fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

fn hint(m: &[u8], b: usize) -> BigInt {

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {

fn decodeint(s: &[u8], b: usize) -> BigInt {

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {

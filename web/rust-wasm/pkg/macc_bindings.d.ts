/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} data
* @returns {string}
*/
export function to_hex(data: Uint8Array): string;
/**
* @param {number} block_height
* @returns {string}
*/
export function calculate_mining_reward(block_height: number): string;
/**
* @returns {BigInt}
*/
export function current_time(): BigInt;
/**
* @param {string} tx_str
* @returns {string | undefined}
*/
export function tx_hash(tx_str: string): string | undefined;
/**
* @param {string} tx_str
* @returns {string | undefined}
*/
export function tx_vout_total(tx_str: string): string | undefined;
/**
* @param {string} tx_str
* @param {string} store_str
* @returns {string | undefined}
*/
export function tx_vin_total(tx_str: string, store_str: string): string | undefined;
/**
* @param {string} block_str
* @returns {string | undefined}
*/
export function block_hash(block_str: string): string | undefined;
/**
* @param {string} blockchain_str
* @param {string} hash_str
* @returns {string | undefined}
*/
export function get_tx(blockchain_str: string, hash_str: string): string | undefined;
/**
* @param {string} sk_key
* @returns {string | undefined}
*/
export function get_client(sk_key: string): string | undefined;
/**
* @param {string} store_str
* @param {string} addr
* @returns {string | undefined}
*/
export function my_utxos(store_str: string, addr: string): string | undefined;
/**
* @param {string} owned_str
* @param {string} sk_key
* @param {string} addr
* @param {string} amount_str
* @returns {string | undefined}
*/
export function send(owned_str: string, sk_key: string, addr: string, amount_str: string): string | undefined;

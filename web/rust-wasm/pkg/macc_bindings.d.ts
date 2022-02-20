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

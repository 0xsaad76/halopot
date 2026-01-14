import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

// Seeds
export const POOL_SEED = "halopot";
export const TICKET_MINT_SEED = "ticket_mint";
export const TICKET_SEED = "ticket";

// Amounts
export const TICKET_PRICE_SOL = 1.0;
export const WRONG_PRICE_SOL = 0.5;
export const MOCK_YIELD_SOL = 2.0;

// Errors
export const ERR_INCORRECT_PRICE = "IncorrectTicketPrice";

export const MARINADE = {
    PROGRAM_ID: new PublicKey("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"),
    STATE: new PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"),
    MSOL_MINT: new PublicKey("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"),
    MSOL_MINT_AUTH: new PublicKey("3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM"),
    SOL_LEG: new PublicKey("UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q"),
    MSOL_LEG: new PublicKey("7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE"),
    MSOL_LEG_AUTH: new PublicKey("EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL"),
    RESERVE: new PublicKey("Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN"),
    TREASURY_MSOL: new PublicKey("8ZUcztoAEhpAeC2ixWewJKQJsSUGYSGPVAjkhDJYf5Gd"), // Devnet Treasury
};

export const PUBLIC_KEY = "yjvBL3F2SkAwP5n3h6VP3UHorWbzEudxgTZWvzcbyEt"
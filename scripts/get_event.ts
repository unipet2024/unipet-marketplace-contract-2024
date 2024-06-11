import { BorshCoder, EventParser } from "@coral-xyz/anchor";
import { program, connection } from "./helper";

const eventParser = new EventParser(
  program.programId,
  new BorshCoder(program.idl)
);

const run = async () => {
  // Get transaction from its signature
  const tx = await connection.getTransaction(
    // "3H3Vo1HXV5sSfdg3FvLzdYEWRqpdvZLYJgexXRgDzGFVapp4m5gkCNav7gh3ohw4AsZzXq99dixzapdRQS8Pw5Vp",
    // "54RFdiJghaVS67goyWrgxoVcHke2s3Snh2UiJJG3zzwAJHYo7AEor44D3djbvae9LTYbUtwPrMcJsBQn13o7CeGP",
    "5t5aLDLu8eJT6G55DoTBuPTKZmWc78Sv5kSGH7CK9LuyDDjkut3kcUnePKY8PVVqUswfwNSVWMcTUQMfZk4Rn4G8",
    {
      commitment: "confirmed",
    }
  );

  const events = eventParser.parseLogs(tx.meta.logMessages);
  for (let event of events) {
    console.log(event);
  }
};

run();

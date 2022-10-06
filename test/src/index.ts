import { readFileSync, writeFileSync } from "fs";

type TOutput = {
  id: string;
  transactionHash: string;
  timestamp: string;
  nonce: number;
  value: number;
  status: number;
  gasCost: number;
  gasFeeCap: number;
  gasLimit: number;
  gasPremium: number;
  addressFrom: string;
  addressTo: string;
  function: string;
  cid: string;
  block: string;
  heightBlock: number;
  methodType: string;
  methodName: string;
  version: string;
  params: string;
  stateDifference: {
    addressFrom: string;
    addressTo: string;
    balanceFrom: string;
    balanceTo: string;
  };
  internalTransactions: TOutput[];
};

type Contract = {
  id: string;
  network: string;
  isVerified: true;
  numberOfTransactions: number;
  totalValueLocked: number;
  hashValue: string;
  project: {
    id: string;
    name: string;
    numberOfContracts: number;
    numberOfTransactions: number;
    totalValueLocked: number;
  };
  sourceCode: string;
  compilerVersion: string;
  transactions: TOutput[];
};

type CID = Record<"/", string>;

type IMessage = {
  Version: number;
  To: string;
  From: string;
  Nonce: number | null;
  Value: string | null;
  GasLimit: number | null;
  GasFeeCap: string | null;
  GasPremium: string | null;
  Method: number;
  Params: string | null;
  CID: CID;
};

type FlowMessageRct = {
  ExitCode: number | null;
  Return: string | null;
  GasUsed: number | null;
  DecodedReturn: unknown | null;
};

type IFlowMessage = {
  Cid: string;
  Message: IMessage;
  Height: number | null;
  BlockCid: string | null;
  MessageRct: FlowMessageRct | null;
  DecodedParams: unknown | null;
  SubCallOf: string | null;
  Action: "Installed" | "Created" | "MethodInvoke";
};

type FMap = Record<string, Contract>;

const sync = (inJson: string): FMap => {
  const v: string[] = readFileSync(inJson)
    .toString()
    // .replaceAll(',"zz":"flow"}', "}\n")
    .split("\n");

  const data: IFlowMessage[] = v
    .filter((v) => !!v.trim().length)
    .map((m) => JSON.parse(m));

  const map: FMap = {};

  data.forEach((message) => {
    if (message.Message.From.startsWith("t3") && message.Message.To === "t01") {
      const sub_call = data.find((m) => {
        return m.SubCallOf === message.Cid;
      });

      if (sub_call) {
        map[sub_call.Message.To] = {
          compilerVersion: "v0",
          hashValue: message.DecodedParams
            ? (message.DecodedParams as any)?.CodeCID["/"]
            : "",
          id: sub_call.Message.To,
          isVerified: true,
          network: "devnet",
          numberOfTransactions: 0,
          project: {
            id: "#demo",
            name: "Demo Project",
            numberOfContracts: 1,
            numberOfTransactions: 1,
            totalValueLocked: 0,
          },
          sourceCode: "<unknown>",
          totalValueLocked: 0,
          transactions: [messageToTransaction(message, sub_call)],
        };
      }

      return;
    }

    if (message.Message.From.startsWith("t3") && !message.SubCallOf) {
      if (map[message.Message.To]) {
        map[message.Message.To].transactions.push(
          messageToTransaction(
            message,
            ...data.filter((m) => m.SubCallOf === message.Cid)
          )
        );
      }
    }
  });

  return map;
};

function messageToTransaction(
  message: IFlowMessage,
  ...internal: IFlowMessage[]
): TOutput {
  return {
    addressFrom: message.Message.From,
    addressTo: message.Message.To,
    block: message.BlockCid as string,
    cid: message.Cid,
    function: "<unknown>",
    gasCost: message.MessageRct?.GasUsed || 0,
    gasFeeCap: +(message.Message.GasFeeCap || 0),
    gasLimit: +(message.Message.GasLimit || 0),
    gasPremium: +(message.Message.GasPremium || 0),
    heightBlock: +(message.Height || 0),
    id: message.Cid,
    internalTransactions: internal.map((m) => messageToTransaction(m)),
    methodName: message.Message.Method.toString(),
    methodType: message.Message.Method.toString(),
    nonce: message.Message.Nonce || 0,
    params: JSON.stringify(message.DecodedParams),
    stateDifference: {
      addressFrom: "",
      addressTo: "",
      balanceFrom: "",
      balanceTo: "",
    },
    status: message.MessageRct?.ExitCode || 0,
    timestamp: Date.now().toString(),
    transactionHash: message.Cid,
    value: +(message.Message.Value || 0),
    version: message.Message.Version.toString(),
  };
}

setInterval(() => {
  const outJson = process.argv[2];
  const inJson = process.argv[3];

  if (!outJson) throw "[FIRST_ARG] JSON file output file path is required";
  if (!inJson)
    throw "[SECOND_ARG] JSON input file path is required (contract transactions/blocks)";

  try {
    const data = sync(inJson);

    writeFileSync(outJson, JSON.stringify(Object.values(data)));
    console.log("synced data...");
  } catch (e) {
    writeFileSync(outJson, JSON.stringify([]));
    console.log("sync(err) data...", e);
  }
}, 1000);

import * as borsh from "borsh";

export enum InstructionTypeEnum {
  CalcType = 0,
}

export enum CalcTypeEnum {
  Increment = 0,
  Decrement = 1,
  Multiply = 2,
  Divide = 3,
}

export class InstructionType {
  tag: number;
  calcTypeTag: number;
  value: number;

  constructor(calcType: CalcTypeEnum, value: number) {
    this.tag = InstructionTypeEnum.CalcType;
    this.calcTypeTag = calcType;
    this.value = value;
  }
}

export class AccountData {
  calc_val: number;

  constructor(fields: { calc_val: number }) {
    this.calc_val = fields.calc_val;
  }
}

export const instructionTypeSchema: borsh.Schema = {
  struct: {
    tag: "u8",
    calcTypeTag: "u8",
    value: "u32",
  },
};

export const accountDataSchema: borsh.Schema = {
  struct: {
    calc_val: "u32",
  },
};

export const ACCOUNT_DATA_LENGTH = borsh.serialize(
  accountDataSchema,
  new AccountData({ calc_val: 200 }),
).length;

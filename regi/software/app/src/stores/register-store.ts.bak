import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { RegisterType } from '../schema/register'
import { CombinedRegistersType } from '../schema/combined-register'
import { MockType } from '../schema/mock'
import { BitConditionType, InterpreterRegisterBitsType, InterpreterType, InterpreterTypeEnum } from '../schema/interpreter'
import { eightEmptyBits } from '../empty-objects/register'
import { BitType } from '../schema/bits'
import { move } from '@dnd-kit/helpers'

interface RegisterState {
  registers: RegisterType[]
  combinedRegister: CombinedRegistersType[]
  interpreters: InterpreterType[]
  mocks: MockType[]
}

interface RegisterStore extends RegisterState {
  addNewRegister: () => RegisterType[]

  addNewCombinedRegister: () => CombinedRegistersType[]

  updateRegister: (registerId: number, key: 'name' | 'address' | 'description', data: string) => RegisterType | undefined,

  updateRegisterBitType: (registerId: number, bitId: number, bitType: BitType) => void

  moveCombinedRegister: (combinedId: number, ev: any) => void

  updateRegisterBitResValue: (registerId: number, bitId: number, value: string) => void

  insertRegisterToCombinedRegister: (combinedRegId: number, registerId: number) => void

  removeCombinedRegister: (combinedRegId: number, registerId: number) => void

  reorderCombinedRegister: (combinedRegId: number) => void

  updateCombinedRegister: (combinedId: number, type: 'name' | 'description', data: string) => void

  addNewInterpreter: (registerId: number) => InterpreterType[]

  updateInterpreterBits: (interpreterId: number, registerId: number, bitId: number) => void

  removeInterpreter: (interpreterId: number) => void

  updateInterpreterNameOrDescription: (interpreterId: number, type: 'name' | 'description', data: string) => void

  updateInterpreterType: (interpreterId: number, type: InterpreterTypeEnum) => void

  updateInterpreterFormula: (interpreterId: number, formula: string) => void

  updateInterpreterBitconditions: (interpreterId: number, bitConditions: BitConditionType[]) => void

  updateInterpreterMock: (interpreterId: number, registerId: number, mock: MockType) => void

  updateMockBitValue: (mockId: number, bitId: number, value: string) => void

  getInterpreterRegisterBits: (interpreterId: number, registerId: number) => InterpreterRegisterBitsType | undefined

  getLatestRegisterId: () => number

  getLatestMockId: () => number

  getLatestInterpreterId: () => number

  getLatestCombinedRegisterId: () => number

  getLatestCombinedRegisterOrdinal: (combinedId: number) => number

  getLatestRegisterOrdinal: () => number

  getLatestBitId: (registerId: number) => number | undefined
}

const initialState: RegisterState = {
  registers: [],
  combinedRegister: [],
  mocks: [],
  interpreters: [],
}

export const useRegisterStore = create<RegisterStore>()(
  immer((set, get) => ({
    ...initialState,

    addNewRegister: (): RegisterType[] => {
      const latestRegisterId = get().getLatestRegisterId()
      const latestRegisterOrdinal = get().getLatestRegisterOrdinal()

      const newRegister = {
        register_id: latestRegisterId + 1,
        ordinal: latestRegisterOrdinal + 1,
        name: '',
        description: '',
        address: '',
        bits: [...eightEmptyBits],
        interpreterIds: []
      }

      set((state) => {
        state.registers.push(newRegister)
      })

      return get().registers
    },

    moveCombinedRegister: (combinedId: number, ev: any) => {
      set((state) => {
        const combinedRegisters = state.combinedRegister.find(comb => comb.combined_id === combinedId)
        if (!combinedRegisters) return
        const moveResult = move(combinedRegisters.registers as any[], ev)
        combinedRegisters.registers = moveResult
      })
    },

    updateRegister: (registerId: number, key: 'name' | 'address' | 'description', data: string) => {
      set((state) => {
        const register = state.registers.find(pr => pr.register_id === registerId)
        if (!register) return;

        if (key === 'name')
          register.name = data

        else if (key === 'description')
          register.description = data

        else if (key === 'address')
          register.address = data

      })
      return get().registers.find(reg => reg.register_id === registerId)
    },

    updateRegisterBitType: (registerId: number, bitId: number, bitType: BitType) => {
      set((state) => {
        const register = state.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const bit = register.bits.find(bit => bit.bit_id === bitId)
        if (!bit) return;

        bit.bit_type = bitType
      })
    },

    updateRegisterBitResValue: (registerId: number, bitId: number, value: string) => {
      set((state) => {
        const register = state.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const bit = register.bits.find(bit => bit.bit_id === bitId)
        if (!bit) return;

        bit.reset_val = value
      })
    },

    addNewCombinedRegister: (): CombinedRegistersType[] => {
      const state = get()

      const latestCombinedRegisterId = state.getLatestCombinedRegisterId()

      const newCombinedRegister: CombinedRegistersType = {
        combined_id: latestCombinedRegisterId + 1,
        registers: [],
        name: `Combined Register ${latestCombinedRegisterId + 1}`
      }

      set((state) => {
        state.combinedRegister.push(newCombinedRegister)
      })

      return get().combinedRegister
    },

    insertRegisterToCombinedRegister: (combinedRegId: number, registerId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        const currentRegistersIds = combinedRegister.registers.map((reg) => reg.register_id)
        const latestCombinedRegisterOrdinal = Math.max(...combinedRegister.registers.map(reg => reg.ordinal))
        if (!currentRegistersIds.includes(registerId)) {
          combinedRegister.registers.push({
            register_id: registerId,
            ordinal: latestCombinedRegisterOrdinal + 1
          })
        }
      })

      get().reorderCombinedRegister(registerId)
    },

    removeCombinedRegister: (combinedRegId: number, registerId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        const currentRegistersIds = combinedRegister.registers.map((reg) => reg.register_id)
        if (currentRegistersIds.includes(registerId)) {
          combinedRegister.registers = combinedRegister.registers.filter(reg => reg.register_id !== registerId)
        }
      })
    },

    updateCombinedRegister: (combinedRegId: number, type: 'name' | 'description', data: string) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;
        if (type === 'name')
          combinedRegister.name = data
      })
    },

    reorderCombinedRegister: (combinedRegId: number) => {
      set((state) => {
        const combinedRegister = state.combinedRegister.find(comb => comb.combined_id === combinedRegId)
        if (!combinedRegister) return;

        combinedRegister.registers = combinedRegister.registers
          .slice()
          .sort((a, b) => a.ordinal - b.ordinal)
          .map((reg, index) => ({
            ...reg,
            ordinal: index
          }))

      })
    },

    getLatestRegisterId: (): number => {
      const state = get()

      if (state.registers.length <= 0) return 0
      const lastestRegisterId = Math.max(...state.registers.map((reg) => reg.register_id))

      return lastestRegisterId
    },

    getLatestRegisterOrdinal: (): number => {
      const state = get()

      if (state.registers.length <= 0) return 0
      const latestRegisterOrdinal = Math.max(...state.registers.map((reg) => reg.ordinal))

      return latestRegisterOrdinal
    },

    getLatestMockId: (): number => {
      const state = get()

      if (state.mocks.length <= 0) return 0
      const latestMockId = Math.max(...state.mocks.map((mock) => mock.mock_id))

      return latestMockId
    },

    getLatestInterpreterId: (): number => {
      const state = get()

      if (state.interpreters.length <= 0) return 0
      const latestInterpreterId = Math.max(...state.interpreters.map((int) => int.interpreter_id))

      return latestInterpreterId
    },

    getLatestBitId: (registerId: number): number | undefined => {
      const state = get()
      const register = state.registers.find((pr) => pr.register_id === registerId)

      if (!register) return undefined

      if (register.bits.length <= 0) return 0
      const latestBitsId = Math.max(...register.bits.map((bit) => bit.bit_id))

      return latestBitsId
    },

    getLatestCombinedRegisterId: (): number => {
      const combinedRegister = get().combinedRegister

      if (combinedRegister.length <= 0) return 0
      const latestCombinedRegisterId = Math.max(...combinedRegister.map((comb) => comb.combined_id))
      return latestCombinedRegisterId
    },

    getLatestCombinedRegisterOrdinal: (combinedId: number): number => {
      const combinedRegister = get().combinedRegister.find((comb) => comb.combined_id === combinedId)
      if (!combinedRegister) return 0

      if (combinedRegister.registers.length <= 0) return 0
      const latestCombinedRegisterOrdinal = Math.max(...combinedRegister.registers.map(reg => reg.ordinal))
      return latestCombinedRegisterOrdinal
    },

    addNewInterpreter: (registerId: number): InterpreterType[] => {
      const state = get()
      const interpreters = state.interpreters;
      const interpreterId = interpreters.length <= 0 ? 0 : Math.max(...interpreters.map(int => int.interpreter_id))
      const newInterpreterId = interpreterId + 1

      const latestMockId = state.getLatestMockId()

      const newMock: MockType = {
        register_id: registerId,
        interpreter_id: newInterpreterId,
        mock_id: latestMockId + 1,
        datas: []
      }

      const newInterpreter: InterpreterType = {
        name: `register ${registerId} interpreter ${newInterpreterId}`,
        description: '',
        registers: [{
          register_id: registerId,
          bit_ids: []
        }],
        interpreter_id: newInterpreterId,
        type: 'Formula'
      }

      set((state) => {
        state.interpreters.push(newInterpreter)
        state.mocks.push(newMock)
      })

      return get().interpreters
    },

    removeInterpreter: (interpreterId: number) => {
      set((state) => {
        state.interpreters = state.interpreters.filter(int => int.interpreter_id !== interpreterId)
      })
    },

    updateInterpreterBits: (interpreterId: number, registerId: number, bitId: number): void => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;

        const register = interpreter.registers.find(reg => reg.register_id === registerId)
        if (!register) return;

        const mock = state.mocks.find(mock => mock.interpreter_id === interpreterId && mock.register_id === registerId)
        if (!mock) return

        const allInterpreterBitsIds = register.bit_ids

        if (!allInterpreterBitsIds.includes(bitId)) {
          register.bit_ids.push(bitId)
          mock.datas.push({
            bit_id: bitId,
            value: '0',
          })
        }
        else {
          register.bit_ids = register.bit_ids.filter(bit => bit !== bitId)
          mock.datas = mock.datas.filter(mock => mock.bit_id !== bitId)
        }
      })
    },


    updateInterpreterType: (interpreterId: number, type: InterpreterTypeEnum) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;

        interpreter.type = type
      })
    },

    updateInterpreterFormula: (interpreterId: number, formula: string) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;
        interpreter.formula = formula
      })
    },

    updateInterpreterBitconditions: (interpreterId: number, bitConditions: BitConditionType[]) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return;

        interpreter.bitConditions = bitConditions
      })
    },

    updateInterpreterMock: (interpreterId: number, registerId: number, mock: MockType) => {
      set((state) => {

        let stateMockIndex = state.mocks.findIndex(mock => mock.interpreter_id === interpreterId && mock.register_id === registerId)

        if (stateMockIndex === -1) {
          state.mocks.push(mock)
        }
        else {
          state.mocks[stateMockIndex] = mock
        }
      })
    },

    getInterpreterRegisterBits: (interpreterId: number, registerId: number): InterpreterRegisterBitsType | undefined => {
      const interpreter = get().interpreters.find(int => int.interpreter_id === interpreterId)
      if (!interpreter) return undefined

      const register = interpreter.registers.find(reg => reg.register_id === registerId)
      return register
    },

    updateInterpreterNameOrDescription: (interpreterId: number, type: 'name' | 'description', data: string) => {
      set((state) => {
        const interpreter = state.interpreters.find(int => int.interpreter_id === interpreterId)
        if (!interpreter) return

        if (type === 'name') interpreter.name = data
        if (type === 'description') interpreter.description = data
      })
    },

    updateMockBitValue: (mockId: number, bitId: number, value: string) => {
      set((state) => {
        const mock = state.mocks.find(mock => mock.mock_id === mockId)
        const mockIndex = state.mocks.findIndex(mock => mock.mock_id === mockId)
        if (mockIndex === -1 || !mock) return

        const mockBitIndex = mock.datas.findIndex(bit => bit.bit_id === bitId)
        const mockBit = mock.datas.find(bit => bit.bit_id === bitId)

        if (mockBitIndex === -1 || !mockBit) {
          mock.datas.push({
            bit_id: bitId,
            value,
          })
        }

        mock.datas[mockBitIndex] = {
          bit_id: bitId,
          value
        }
      })
    }

  }))
)

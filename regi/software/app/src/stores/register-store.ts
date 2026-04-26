import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { RegisterType } from '../schema/register'
import { CombinedRegistersType } from '../schema/combined-register'
import { MockType } from '../schema/mock'
import { InterpreterRegisterBitsType, InterpreterType } from '../schema/interpreter'
import { TranslatorType } from '../schema/translator'
import { eightEmptyBits } from '../empty-objects/register'
import { BitType, bitTypes } from '../schema/bits'
import { move } from '@dnd-kit/helpers'
import { current } from 'immer'

interface RegisterState {
  registers: RegisterType[]
  combinedRegister: CombinedRegistersType[]
  mocks: MockType[]
  interpreters: InterpreterType[]
  translators: TranslatorType[]
}

interface RegisterStore extends RegisterState {
  addNewRegister: () => RegisterType[]

  addNewCombinedRegister: () => CombinedRegistersType[]

  addNewTranslator: () => TranslatorType

  updateRegister: (registerId: number, key: 'name' | 'description', data: string) => RegisterType | undefined,

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

  getInterpreterRegisterBits: (interpreterId: number, registerId: number) => InterpreterRegisterBitsType | undefined

  getLatestRegisterId: () => number

  getLatestMockId: () => number

  getLatestTranslatorId: () => number

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
  translators: []
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
        address: 0,
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

    updateRegister: (registerId: number, key: 'name' | 'description', data: string) => {
      set((state) => {
        const register = state.registers.find(pr => pr.register_id === registerId)
        if (!register) return;

        if (key === 'name') {
          register.name = data
        }
        else if (key === 'description') {
          register.description = data
        }

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

    getLatestTranslatorId: (): number => {
      const state = get()

      if (state.translators.length <= 0) return 0
      const latestTranslatorId = Math.max(...state.translators.map((tr) => tr.translator_id))

      return latestTranslatorId
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
      const interpreters = get().interpreters;

      const interpreterId = interpreters.length <= 0 ? 0 : Math.max(...interpreters.map(int => int.interpreter_id))

      const newTranslator = get().addNewTranslator()

      const newInterpreter: InterpreterType = {
        name: `register ${registerId} interpreter ${interpreterId + 1}`,
        description: '',
        registers: [{
          register_id: registerId,
          bit_ids: []
        }],
        interpreter_id: interpreterId + 1,
        translator_id: newTranslator.translator_id
      }

      set((state) => {
        state.interpreters.push(newInterpreter)
      })

      return get().interpreters
    },

    addNewTranslator: (): TranslatorType => {
      const translators = get().translators

      const latestTranslatorId = translators.length <= 0 ? 0 :
        Math.max(...translators.map(tr => tr.translator_id))

      const newTranslator: TranslatorType = {
        translator_id: latestTranslatorId + 1,
        translator_type: 'Pair',
        formula: null,
        keyPairs: null
      }

      set((state) => {
        state.translators.push(newTranslator)
      })

      return newTranslator
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

        const allInterpreterBitsIds = register.bit_ids

        if (!allInterpreterBitsIds.includes(bitId)) {
          register.bit_ids.push(bitId)
        }
        else {
          register.bit_ids = register.bit_ids.filter(bit => bit !== bitId)
        }
        console.log("pushing bitid: ", current(register))
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
    }

  }))
)

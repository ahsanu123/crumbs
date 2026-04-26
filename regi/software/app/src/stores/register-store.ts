import { create } from 'zustand'
import { immer } from 'zustand/middleware/immer'
import { RegisterType } from '../schema/register'
import { CombinedRegistersType } from '../schema/combined-register'
import { MockType } from '../schema/mock'
import { InterpreterType } from '../schema/interpreter'
import { TranslatorType } from '../schema/translator'
import { eightEmptyBits } from '../empty-objects/register'
import { BitType, bitTypes } from '../schema/bits'
import { move } from '@dnd-kit/helpers'

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

  updateRegister: (registerId: number, key: 'name' | 'description', data: string) => RegisterType | undefined,

  updateRegisterBitType: (registerId: number, bitId: number, bitType: BitType) => void

  moveCombinedRegister: (combinedId: number, ev: any) => void

  updateRegisterBitResValue: (registerId: number, bitId: number, value: string) => void

  insertRegisterToCombinedRegister: (combinedRegId: number, registerId: number) => void

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
        console.log("moved result", moveResult)
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
        registers: []
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
        if (!currentRegistersIds.includes(registerId)) {
          combinedRegister.registers.push({
            register_id: registerId,
            ordinal: 0
          })
        }

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
    }

  }))
)

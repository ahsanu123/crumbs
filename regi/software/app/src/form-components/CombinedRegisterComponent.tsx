import React, { useMemo } from "react"
import { CombinedRegistersType } from "../schema/combined-register"
import { useRegisterStore } from "../stores/register-store"
import { Button, FormControl, InputLabel, MenuItem, Select, Stack, Typography } from "@mui/material"
import { useSortable, } from '@dnd-kit/react/sortable'
import { DragDropProvider } from '@dnd-kit/react'
import { RegisterType } from "../schema/register";
import { move } from '@dnd-kit/helpers'

interface RegisterSortableProps {
  register: RegisterType,
  index: number
}
const RegisterSortable = (props: RegisterSortableProps) => {
  const {
    register,
    index
  } = props

  const { ref } = useSortable({ id: register.register_id, index });

  return (
    <Button
      variant="contained"
      ref={ref}
      sx={{
        margin: "5px 15px"
      }}
    >
      {register.name}
    </Button>
  );
}

interface CombinedRegisterListProps {
  combinedId: number,
  registers: RegisterType[]
}

function CombinedRegisterList(props: CombinedRegisterListProps) {
  const {
    combinedId,
    registers
  } = props

  const moveCombinedRegister = useRegisterStore(store => store.moveCombinedRegister)

  return (
    <DragDropProvider
      onDragEnd={(ev) => {
        console.log("after onDragEnd")
        moveCombinedRegister(combinedId, ev)
      }}
    >
      <Stack
        direction={'row'}
        sx={{
          borderRadius: "20px",
          border: "1px solid gray"
        }}
      >
        {registers.map((reg, index) =>
          <RegisterSortable
            key={reg.register_id}
            register={reg}
            index={index}
          />
        )}
      </Stack>
    </DragDropProvider>
  );
}

interface CombinedRegisterProps {
  combinedRegister: CombinedRegistersType
}

export function CombinedRegister(props: CombinedRegisterProps) {
  const {
    combinedRegister
  } = props

  const registers = useRegisterStore(store => store.registers)
  const allCombinedRegister = useRegisterStore(store => store.combinedRegister)
  const insertRegisterToCombinedRegister = useRegisterStore(store => store.insertRegisterToCombinedRegister)

  const notIncludedRegister = useMemo(() => {
    const allRegisters = allCombinedRegister.map(comb => comb.registers).flat()
    const allRegisterIds = allRegisters.map(reg => reg.register_id)

    return registers.filter(reg => !allRegisterIds.includes(reg.register_id))
  }, [registers, allCombinedRegister])

  const includedRegister = useMemo(() => {
    const registerIds = combinedRegister.registers.map(reg => reg.register_id)
    return registers.filter(reg => registerIds.includes(reg.register_id))
  }, [registers, combinedRegister])

  const handleOnAddRegisterToCombinedRegister = (registerId: number) => {
    insertRegisterToCombinedRegister(combinedRegister.combined_id, registerId)
  }

  return (
    <>
      <CombinedRegisterList
        combinedId={combinedRegister.combined_id}
        registers={includedRegister}
      />

      <FormControl fullWidth>
        <InputLabel id="register-select">Register</InputLabel>
        <Select
          labelId="register-select"
          id="register-select"
          label="Register"
          onChange={(ev) => handleOnAddRegisterToCombinedRegister(parseInt(ev.target.value as string))}
        >
          {notIncludedRegister.map((reg) => (
            <MenuItem value={reg.register_id}>{reg.name}</MenuItem>
          ))}
        </Select>
      </FormControl>
    </>
  )
}

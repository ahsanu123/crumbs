import { useMemo } from "react"
import { CombinedRegistersType } from "../schema/combined-register"
import { useRegisterStore } from "../stores/register-store"
import { FormControl, InputLabel, MenuItem, Select, Stack } from "@mui/material"
import { CombinedRegisterList } from "./combined-register-component/CombinedRegisterList"

interface CombinedRegisterComponentProps {
  combinedRegister: CombinedRegistersType
}

export function CombinedRegisterComponent(props: CombinedRegisterComponentProps) {
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
    <Stack
      sx={{
        marginTop: "30px"
      }}
    >

      <FormControl fullWidth>
        <InputLabel id="register-select">Register</InputLabel>
        <Select
          labelId="register-select"
          id="register-select"
          label="Register To Add"
          disabled={notIncludedRegister.length === 0}
          onChange={(ev) => handleOnAddRegisterToCombinedRegister(parseInt(ev.target.value as string))}
        >
          {notIncludedRegister.map((reg) => (
            <MenuItem value={reg.register_id}>{reg.name}</MenuItem>
          ))}
        </Select>
      </FormControl>

      <CombinedRegisterList
        combinedId={combinedRegister.combined_id}
        name={combinedRegister.name}
        registers={includedRegister}
      />
    </Stack>
  )
}



import { DragDropProvider } from '@dnd-kit/react'
import { Stack, TextField, Typography } from '@mui/material';
import { RegisterType } from '../../schema/register';
import { useRegisterStore } from '../../stores/register-store';
import { RegisterSortable } from './RegisterSortable';

interface CombinedRegisterListProps {
  combinedId: number,
  name: string,
  registers: RegisterType[]
}

export const CombinedRegisterList = (props: CombinedRegisterListProps) => {
  const {
    combinedId,
    name,
    registers
  } = props

  const moveCombinedRegister = useRegisterStore(store => store.moveCombinedRegister)
  const removeCombinedRegister = useRegisterStore(store => store.removeCombinedRegister)
  const updateCombinedRegister = useRegisterStore(store => store.updateCombinedRegister)

  return (
    <>
      <Stack
        sx={{
          margin: "30px 0 10px 0",
          gap: "30px"
        }}
      >
        <TextField
          value={name}
          label="combined register name"
          placeholder="create combined register name"
          onChange={(ev) => updateCombinedRegister(combinedId, 'name', ev.target.value)}
        />

        <DragDropProvider
          onDragEnd={(ev) => moveCombinedRegister(combinedId, ev)}
        >
          <Stack
            direction={'row'}
            sx={{
              backgroundColor: '#e8eaf6',
              borderRadius: "10px",
              border: "1px solid gray",
              minHeight: "50px"
            }}
          >
            {registers.length === 0 && (
              <Typography
                sx={{
                  alignSelf: 'center',
                  marginLeft: "20px"
                }}
              >
                Empty, please select register to combine.
              </Typography>
            )}

            {registers.map((reg, index) =>
              <RegisterSortable
                key={reg.register_id}
                register={reg}
                index={index}
                onRemoveRegister={(regId) => removeCombinedRegister(combinedId, regId)}
              />
            )}
          </Stack>
        </DragDropProvider>
      </Stack>

      <Stack
        direction={'row'}
        sx={{
          justifyContent: 'space-between',
          margin: "0 0 25px 0"
        }}
      >
        <Typography
          variant="subtitle2"
        >
          MSB
        </Typography>

        <Typography
          variant="subtitle2"
        >
          LSB
        </Typography>
      </Stack>
    </>
  );
}

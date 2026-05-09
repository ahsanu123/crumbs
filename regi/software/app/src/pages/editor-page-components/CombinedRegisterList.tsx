import { DragDropProvider } from '@dnd-kit/react'
import { Stack, TextField, Typography } from '@mui/material';
import { RegisterSortable } from './RegisterSortable';
import { RegisterSchema } from '../../schema/register';
import { useEditorPageStore } from '../../stores';
import { CombinedRegisterSchema } from '../../schema/combined-register';

interface CombinedRegisterListProps {
  combinedRegister: CombinedRegisterSchema
  registers: RegisterSchema[]
}

export const CombinedRegisterList = (props: CombinedRegisterListProps) => {
  const {
    combinedRegister,
    registers
  } = props

  const moveCombinedRegister = useEditorPageStore(store => store.moveCombinedRegister)
  const removeCombinedRegisterMember = useEditorPageStore(store => store.removeCombinedRegisterMember)
  const updateCombinedRegister = useEditorPageStore(store => store.updateCombinedRegister)
  const orderedCombinedRegisterMember = useEditorPageStore(store => store.getOrderedCombinedRegisterMember)

  return (
    <Stack>
      <Stack
        sx={{
          margin: "30px 0 10px 0",
          gap: "30px"
        }}
      >
        <TextField
          value={combinedRegister.name}
          label="combined register name"
          placeholder="create combined register name"
          onChange={(ev) => updateCombinedRegister(combinedRegister.combined_id, 'name', ev.target.value)}
        />

        <DragDropProvider
          onDragEnd={(ev) => moveCombinedRegister(combinedRegister.combined_id, ev)}
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
                onRemoveRegister={(regId) => removeCombinedRegisterMember(combinedRegister.combined_id, regId)}
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

      {orderedCombinedRegisterMember(combinedRegister.combined_id).map((reg) => (
        <Typography>
          {reg.name} ordinal: {reg.ordinal}
        </Typography>
      ))}
    </Stack>
  );
}

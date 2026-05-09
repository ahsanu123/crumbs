import { useSortable, } from '@dnd-kit/react/sortable'
import { IoIosRemoveCircleOutline } from "react-icons/io";
import { MdDragIndicator } from "react-icons/md";
import { useRef } from 'react';
import { Box, Button, Stack, Typography } from '@mui/material';
import { RegisterSchema } from '../../schema/register';

interface RegisterSortableProps {
  register: RegisterSchema,
  index: number
  onRemoveRegister: (regId: number) => void
}

export const RegisterSortable = (props: RegisterSortableProps) => {
  const {
    onRemoveRegister,
    register,
    index
  } = props

  const handleRef = useRef(null)
  const { ref } = useSortable({
    id: register.register_id,
    index,
    handle: handleRef
  });

  return (
    <Box
      ref={ref}
      sx={{
        backgroundColor: 'white',
        border: "1px solid gray",
        borderRadius: "10px",
        padding: "5px 15px",
        margin: "15px 15px"
      }}
    >
      <Stack direction={'row'}>
        <Typography
          variant="subtitle1"
        >
          {register.name}
        </Typography>
        <Button
          sx={{
            padding: "0"
          }}
          ref={handleRef}
        >
          <MdDragIndicator size={24} />
        </Button>

        <Button
          sx={{
            padding: "0"
          }}
          onClick={() => onRemoveRegister(register.register_id)}
        >
          <IoIosRemoveCircleOutline size={24} />
        </Button>

      </Stack>
    </Box>
  );
}

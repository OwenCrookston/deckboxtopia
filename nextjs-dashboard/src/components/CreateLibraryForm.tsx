import { FormControl, FormHelperText, Input, InputLabel } from '@mui/material';

export const CreateLibraryForm = () => {
  return (
    <FormControl>
      <InputLabel htmlFor="my-input">Email address</InputLabel>
      <Input id="my-input" aria-describedby="my-helper-text" />
      <FormHelperText id="my-helper-text">We&aposll never share your email.</FormHelperText>
    </FormControl>
  );
}
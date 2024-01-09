"use client"

import { CreateLibraryForm } from "@/components/CreateLibraryForm"
import { Container, Grid } from "@mui/material";

export default function CreateLibraries () {
  // Page for creating Library


  return (
    <Grid flexDirection="column" alignContent={"center"} justifyContent={"center"}>
      <CreateLibraryForm />
      <>Submit</>
    </Grid>
  );
};
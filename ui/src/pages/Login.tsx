import { Component } from "solid-js";
import Container from "@suid/material/Container";
import Box from "@suid/material/Box";
import TextField from "@suid/material/TextField";
import Button from "@suid/material/Button";
import Paper from "@suid/material/Paper";
import Typography from "@suid/material/Typography";
import Divider from "@suid/material/Divider";

const Login: Component = () => {
  return (
    <Container maxWidth="sm" sx={{ width: "fit-content" }}>
      <Paper sx={{ mt: "20vh" }} elevation={4}>
        <Box
          component="form"
          sx={{
            px: 5,
            py: 2,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            "& > :not(style)": { my: 1.5 },
          }}
          noValidate
        >
          <Typography variant="h5">Pentaract Account</Typography>
          <Divider />
          <TextField label="Username" variant="standard" />
          <TextField label="Password" variant="standard" type="password" />
          <Divider />
          <Button variant="contained">Login</Button>
        </Box>
      </Paper>
    </Container>
  );
};

export default Login;

"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button, Input, Container } from "@mui/material";

export default function ClientComponent({ initialValue }: { initialValue?: string }) {
  const [inputValue, setInputValue] = useState(initialValue || "");
  const [displayValue, setDisplayValue] = useState(initialValue || "");
  const router = useRouter();

  const handleButtonClick: () => void = () => {
    if (inputValue) {
      router.push(`/?input=${encodeURIComponent(inputValue)}`);
      setDisplayValue(inputValue);
    }
  };

  return (
    <Container>
      <p>Input Value: {displayValue}</p>
      <Input
        placeholder="Type something..."
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
      />
      <Button
        variant="contained"
        color="primary"
        onClick={handleButtonClick}
      >
        Click Me
      </Button>
    </Container>
  );
}
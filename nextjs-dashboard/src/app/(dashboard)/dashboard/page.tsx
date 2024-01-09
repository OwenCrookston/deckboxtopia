"use client"
import { useEffect } from "react";

export default function Dashboard () {
  // The dashboard page. Is for a user to view their libraries

  const deckId = '5cd7376c-1688-4186-9d0f-2d2e1fa3d812';
  const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL ?? '';
  console.log(`env var: ${process.env.NEXT_PUBLIC_API_BASE_URL}`);

  useEffect(() => {
    fetch(`${apiBaseUrl}/deck/${deckId}/draw?number=1`)
      .then((res) => res.json())
      .then((json) => console.log(JSON.stringify(json)))
      .catch((err) => {
        console.log(`whoops: ${err}`);
      })
  });

  return (
    <div>hi</div>
  );
};
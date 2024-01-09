import { DeckboxtopiaCard } from '@/app/types/card';
import { Button, TextField, FormControl, FormHelperText, Input, InputLabel } from '@mui/material';
import { useState, useEffect } from 'react';
import { CardsToAdd } from './CardsToAdd';

export const CreateLibraryForm = () => {
  const [cardsToUse, setCardsToUse] = useState<DeckboxtopiaCard[]>([]);
  // const [libraryFormState, setLibraryFormState] = useState<{ name?: string, url?: string }>({});
  const [cardName, setCardName] = useState<string>('');
  const [cardUrl, setCardUrl] = useState<string>('');

  const addCard = () => {
    setCardsToUse((cards) => {
      if (cardName && cardUrl) {
        return [...cards, { name: cardName, art_url: cardUrl }];
      }
      return cards;
    });
  };

  //create a card component or even a scrolling card component containing card components and pass in cardsToUse

  return (
    <>
      <FormControl>
        <TextField
          placeholder='Card Name'
          value={cardName}
          variant='outlined'
          id="card-name"
          type='text'
          onChange={(e) => setCardName(e.target.value)}
        />
        <TextField
          placeholder='Card URL'
          value={cardUrl}
          id="card-url"
          variant='outlined'
          type='text'
          onChange={(e) => setCardUrl(e.target.value)}
        />
        <Button variant='outlined' onClick={(e) => {
          e.preventDefault()
          addCard();
        }}>Add Card</Button>
      </FormControl>
      <FormControl>
        {/* <InputLabel htmlFor="library-name">Library Name</InputLabel> */}
        <Input id="library-name" type='text' />
        {/* <InputLabel htmlFor="card-name">Card Name</InputLabel> */}

        {/* <InputLabel htmlFor="card-url">Card Url</InputLabel> */}
        <Button>Create Library</Button>
      </FormControl>
      <CardsToAdd cards={cardsToUse} />
    </>
  );
}

/// ```ignore
/// {
///     "name": "...",
///     "cards": [
///        {"name": "...", "url": "..."}
///     ]
/// }
/// ```
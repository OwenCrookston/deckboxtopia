import { DeckboxtopiaCard } from "@/app/types/card";
import { CardTile } from "./CardTile";

type CardsToAddProps = {
  cards: DeckboxtopiaCard[],
}

export const CardsToAdd = ({ cards }: CardsToAddProps) => {
  const cardTiles = cards.map((card) => <CardTile card={card} />);

  return (
    <>
      {cardTiles}
    </>
  )
};
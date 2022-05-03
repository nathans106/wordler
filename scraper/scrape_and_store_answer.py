from . import scraper
from common import database

answer = scraper.answer()
database.set_answer(answer)
print(f'Todays answer set to {answer}')

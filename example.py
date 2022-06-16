import ocr_rusty as ocr

if __name__ == '__main__':

    text: str = ocr.get_text('./assets/images/text.jpg')
    print(f'Text: {text}')
def check_result(name, expected, got):
    print('Checking {}... '.format(name), end='')
    if got != expected:
        print('Expected {}, Found {}'.format(expected, got))
    else:
        print('Passed')
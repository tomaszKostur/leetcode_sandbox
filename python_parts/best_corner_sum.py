from collections import defaultdict

test_input = [1,2,3,4,5,6,6,7,2,1,1,]
test_input = [1,2,3,1,2,3,1,2,3]

def best_sum(elements):
    reprtition_dict = find_repetitions(elements)
    # print(reprtition_dict)
    orthogonal = find_orthogonal(reprtition_dict)
    # print(orthogonal)
    best_sum = 0
    best_key = None
    for r_key, (idx_begin, idx_end) in orthogonal.items():
        t_sum = calculate_sum(elements, idx_begin, idx_end)
        if t_sum > best_sum:
            best_sum = t_sum
            best_key = r_key
    return (best_sum, orthogonal.get(best_key))



def find_repetitions(elements):
    repetition_dict = defaultdict(list)
    for idx, val in enumerate(elements):
        repetition_dict[val].append(idx)
    # print(repetition_dict)
    # repetition_dict = filter(lambda x: len(x) > 1 ,repetition_dict)
    repetition_dict = {elem : [indexes[0], indexes[-1]] for elem, indexes in repetition_dict.items() if len(indexes) > 1}
    # assuming only positive integers
    return repetition_dict

def find_orthogonal(repetition_dict):
    orthogonal = {}
    for key, indexes in repetition_dict.items():
        ortho = True
        for ortho_indexes in orthogonal.values():
            if indexes[0] >= ortho_indexes[0] and indexes[-1] <= ortho_indexes[-1]:
                ortho = False
        if ortho:
            orthogonal[key] = indexes
    return orthogonal

def calculate_sum(element, idx_begin, idx_end):
    return sum(element[idx_begin:idx_end+1])


def test_devdev():
    best_sum_val = best_sum(test_input)
    print(best_sum_val)

# test_devdev()

def test_split_collections():
    test_data = [2,3,9,9,9,3,4,999,4]
    best_sum_val = best_sum(test_data)
    print(best_sum_val)
    # assert best_sum_val == (999, [6,8])

def test_overlaping_collection():
    test_data = [2,3,9,9,9,3,999,1,9,4]
    best_sum_val = best_sum(test_data)
    print(best_sum_val)

test_split_collections()
test_overlaping_collection()
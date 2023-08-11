#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>

using std::string;
using std::vector;
using std::cin;
using std::cout;
using std::map;

struct Query {
    string type, s;
    size_t ind;
};

class QueryProcessor {
    map<int, vector<string>> M = {
    };
    int bucket_count;
    // store all strings in one vector
    vector<string> elems;
    size_t hash_func(const string& s) const {
        static const size_t multiplier = 263;
        static const size_t prime = 1000000007;
        unsigned long long hash = 0;
        for (int i = static_cast<int> (s.size()) - 1; i >= 0; --i)
            hash = (hash * multiplier + s[i]) % prime;
        return hash % bucket_count;
    }

public:
    explicit QueryProcessor(int bucket_count): bucket_count(bucket_count) {
        M[0] = {};
        M[1] = {};
        M[2] = {};
        M[3] = {};
        M[4] = {};
    }

    Query readQuery() const {
        Query query;
        cin >> query.type;
        if (query.type != "check")
            cin >> query.s;
        else
            cin >> query.ind;
        return query;
    }

    void writeSearchResult(bool was_found) const {
        std::cout << (was_found ? "yes\n" : "no\n");
    }

    void processQuery(const Query& query) {
        if (query.type == "check") {
            vector<string> strs = M[query.ind];
            for (auto elem : strs) {
                std::cout << elem << " ";
            }
            std::cout << "\n";
        } else {
            int hash = hash_func(query.s);
            vector<string> *hash_bucket = &M[hash];
            vector<string>::iterator it = std::find(hash_bucket->begin(), hash_bucket->end(), query.s);
            if (query.type == "find")
                writeSearchResult(it != hash_bucket->end());
            else if (query.type == "add") {
                if (it == hash_bucket->end())
                hash_bucket->insert(hash_bucket->begin(), query.s);
            } else if (query.type == "del") {
                if (it != hash_bucket->end())
                    hash_bucket->erase(it);
            }
            //             cout << query.type << " " << query.s << " ";
            // for (auto elem : *hash_bucket) {
            //     cout << elem << " x ";
            // }
            // cout << "\n";
        }
    }

    void processQueries() {
        int n;
        cin >> n;
        for (int i = 0; i < n; ++i)
            processQuery(readQuery());
    }
};

int main() {
    std::ios_base::sync_with_stdio(false);
    int bucket_count;
    cin >> bucket_count;
    QueryProcessor proc(bucket_count);
    proc.processQueries();
    return 0;
}
